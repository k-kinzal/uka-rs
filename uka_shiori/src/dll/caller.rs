use crate::types::v3;
use libloading::{Library as Lib, Symbol};
use std::alloc::System;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use uka_util::ptr::{OwnedPtr, RawPtr};

/// LoadFunc is the load interface of the SHIORI DLL.
///
/// ```clang
/// extern "C" __declspec(dllexport) BOOL __cdecl load(HGLOBAL h, long len);
/// function load(h: hglobal; len: longint): boolean; cdecl;
/// ```
type LoadFunc = unsafe extern "C" fn(h: isize, len: usize) -> bool;

/// UnloadFunc is the unload interface of the SHIORI DLL.
///
/// ```clang
/// extern "C" __declspec(dllexport) BOOL __cdecl unload();
/// function unload: boolean; cdecl;
/// ```
type UnloadFunc = unsafe extern "C" fn() -> bool;

/// RequestFunc is the request interface of the SHIORI DLL.
///
/// ```clang
/// extern "C" __declspec(dllexport) HGLOBAL __cdecl request(HGLOBAL h, long *len);
/// function request(h: hglobal; var len: longint): hglobal; cdecl; export;
/// ```
type RequestFunc = unsafe extern "C" fn(h: isize, len: *mut usize) -> isize;

/// Library wraps the SHIORI DLL so that Rust can call load/unload/request.
struct Library(Lib);

impl Library {
    const LOAD_SYMBOL: &'static [u8] = b"load";
    const UNLOAD_SYMBOL: &'static [u8] = b"unload";
    const REQUEST_SYMBOL: &'static [u8] = b"request";

    /// new creates a new Library instance.
    ///
    /// # Safety
    ///
    /// This function inherits the behavior of libloading::Library::new.
    ///
    /// See: https://docs.rs/libloading/latest/libloading/struct.Library.html#method.new
    pub unsafe fn new<P: AsRef<OsStr>>(path: P) -> Result<Self, libloading::Error> {
        let lib = Lib::new(path)?;
        let _ = lib.get::<Symbol<LoadFunc>>(Self::LOAD_SYMBOL)?;
        let _ = lib.get::<Symbol<LoadFunc>>(Self::UNLOAD_SYMBOL)?;
        let _ = lib.get::<Symbol<LoadFunc>>(Self::REQUEST_SYMBOL)?;

        Ok(Self(lib))
    }

    /// Call SHIORI DLL load.
    pub fn load(&self, h: isize, len: usize) -> bool {
        unsafe {
            let func = self
                .0
                .get::<Symbol<LoadFunc>>(Self::LOAD_SYMBOL)
                .expect("unreachable: failed get load symbol");
            func(h, len)
        }
    }

    /// Call SHIORI DLL unload.
    pub fn unload(&self) -> bool {
        unsafe {
            let func = self
                .0
                .get::<Symbol<UnloadFunc>>(Self::UNLOAD_SYMBOL)
                .expect("unreachable: failed get unload symbol");
            func()
        }
    }

    /// Call SHIORI DLL request.
    pub fn request(&self, h: isize, len: *mut usize) -> isize {
        unsafe {
            let func = self
                .0
                .get::<Symbol<RequestFunc>>(Self::REQUEST_SYMBOL)
                .expect("unreachable: failed get request symbol");
            func(h, len)
        }
    }
}

/// Error is the error type of the Caller.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to open library: {0}")]
    FailedOpenLibrary(#[from] libloading::Error),

    #[error("failed to canonicalize path: {0}")]
    FailedCanonicalize(#[from] std::io::Error),

    #[error("failed to SHIORI::load")]
    FailedLoad,
}

trait Caller<R> {
    type Response;

    fn call(&self, request: R) -> Self::Response;
}

/// ShioriCaller calls the SHIORI DLL.
/// This is an anti-corruption layer that hides pointers and values and handles them Rust-like.
pub struct ShioriCaller(Library);

impl ShioriCaller {
    /// new creates a new Caller instance.
    ///
    /// # Safety
    ///
    /// This function inherits the behavior of libloading::Library::new.
    ///
    /// See: https://docs.rs/libloading/latest/libloading/struct.Library.html#method.new
    pub unsafe fn open<P: AsRef<OsStr> + Sized>(path: P) -> Result<Self, Error> {
        let dir = fs::canonicalize(Path::new(&path).join(".."))?;
        let lib = Library::new(path)?;
        let len = dir.as_os_str().len();
        let ptr = Box::into_raw(Box::new(dir));

        if lib.load(ptr as isize, len) {
            Ok(Self(lib))
        } else {
            Err(Error::FailedLoad)
        }
    }
}

impl Caller<v3::Request> for ShioriCaller {
    type Response = v3::Response;

    fn call(&self, request: v3::Request) -> v3::Response {
        use v3::IntoResponse;

        let bytes = request.as_bytes();
        let len = bytes.len();
        let h = OwnedPtr::from_vec(bytes).into_raw_slice().as_ptr() as isize;

        let hglobal = self.0.request(h, &len as *const usize as *mut usize);

        let ptr =
            unsafe { RawPtr::<[u8]>::from_raw_address_parts(hglobal, len).to_owned::<System>() };
        match v3::Response::parse(unsafe { ptr.as_slice() }) {
            Ok(response) => response,
            Err(e) => v3::ShioriError::from(e)
                .with_status_code(v3::StatusCode::INTERNAL_SERVER_ERROR)
                .into_response(),
        }
    }
}

impl Drop for ShioriCaller {
    fn drop(&mut self) {
        let _ = self.0.unload();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use std::sync::Mutex;

    // generate library
    static PATH: Lazy<Mutex<PathBuf>> = Lazy::new(|| {
        Command::new("cargo")
            .args(["build", "-p", "ghost"])
            .status()
            .expect("failed to execute cargo build -p ghost");
        let s = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not found");
        let p = Path::new(&s).join("..").join("target").join("debug");
        if fs::metadata(p.join("libghost.dylib")).is_ok() {
            Mutex::new(p.join("libghost.dylib"))
        } else if fs::metadata(p.join("ghost.dll")).is_ok() {
            Mutex::new(p.join("ghost.dll"))
        } else if fs::metadata(p.join("ghost.so")).is_ok() {
            Mutex::new(p.join("ghost.so"))
        } else {
            panic!("ghost not found");
        }
    });

    #[test]
    fn test_caller_request() -> anyhow::Result<()> {
        let path = PATH.lock().expect("lock failed");
        let caller = unsafe { ShioriCaller::open(path.as_path())? };

        let req = v3::Request::builder()
            .method(v3::Method::GET)
            .version(v3::Version::SHIORI_30)
            .build()?;
        let res = caller.call(req);
        assert_eq!(res.status_code(), v3::StatusCode::OK);

        Ok(())
    }

    #[test]
    fn test_caller_request2() -> anyhow::Result<()> {
        let path = PATH.lock().expect("lock failed");
        let caller = unsafe { ShioriCaller::open(path.as_path())? };

        let req = v3::Request::builder()
            .method(v3::Method::GET)
            .version(v3::Version::SHIORI_30)
            .build()?;
        let res = caller.call(req);
        assert_eq!(res.status_code(), v3::StatusCode::OK);

        Ok(())
    }
}
