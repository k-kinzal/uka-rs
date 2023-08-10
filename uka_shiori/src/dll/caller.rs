use crate::dll::caller::Error::FailedUnload;
use crate::types::v3;
use libloading::{Library as Lib, Symbol};
use std::alloc::System;
use std::ffi::OsStr;
use std::path::PathBuf;
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

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to open library: {0}")]
    FailedOpen(#[from] libloading::Error),

    #[error("failed to load")]
    FailedLoad,

    #[error("failed to unload")]
    FailedUnload,
}

/// The Caller calls the SHIORI DLL.
/// This is an anti-corruption layer that hides pointers and values and handles them Rust-like.
pub struct Caller(Library);

impl Caller {
    /// new creates a new Caller instance.
    ///
    /// # Safety
    ///
    /// This function inherits the behavior of libloading::Library::new.
    ///
    /// See: https://docs.rs/libloading/latest/libloading/struct.Library.html#method.new
    pub unsafe fn open<P: AsRef<OsStr>>(path: P) -> Result<Self, Error> {
        Ok(Self(Library::new(path)?))
    }

    /// Call SHIORI DLL load.
    pub fn load(&self, path: PathBuf) -> Result<(), Error> {
        let len = path.as_os_str().len();
        let s = Box::into_raw(Box::new(path));

        if self.0.load(s as isize, len) {
            Ok(())
        } else {
            Err(Error::FailedLoad)
        }
    }

    /// Call SHIORI DLL unload.
    pub fn unload(&self) -> Result<(), Error> {
        if self.0.unload() {
            Ok(())
        } else {
            Err(FailedUnload)
        }
    }

    /// Call SHIORI DLL request.
    pub fn request(&self, request: v3::Request) -> v3::Response {
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

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;
    use std::path::Path;
    use std::process::Command;

    // generate library
    static PATH: Lazy<PathBuf> = Lazy::new(|| {
        Command::new("cargo")
            .args(&["build", "-p", "ghost"])
            .status()
            .expect("failed to execute cargo build -p ghost");
        let s = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not found");
        let p = Path::new(&s).join("..").join("target").join("debug");
        if std::fs::metadata(&p.join("libghost.dylib")).is_ok() {
            p.join("libghost.dylib")
        } else if std::fs::metadata(&p.join("ghost.dll")).is_ok() {
            p.join("ghost.dll")
        } else if std::fs::metadata(&p.join("ghost.so")).is_ok() {
            p.join("ghost.so")
        } else {
            panic!("ghost not found");
        }
    });

    #[test]
    fn test_caller_load() -> anyhow::Result<()> {
        let path = PATH.clone();
        let caller = unsafe { Caller::open(&path)? };

        let res = caller.load(path.join(".."));
        assert!(res.is_ok(), "{}", res.err().unwrap());

        Ok(())
    }

    // #[test]
    // fn it_works() -> anyhow::Result<()> {
    //     let path = PATH.clone();
    //     let caller = unsafe { Caller::open(&path) };
    //     caller
    //         .load(path.join(".."))
    //         .map_err(|e| anyhow::anyhow!(e))?;
    //
    //     let req = v3::Request::builder()
    //         .method(v3::Method::GET)
    //         .version(v3::Version::SHIORI_30)
    //         .build()?;
    //     let res = caller.request(req);
    //
    //     Ok(())
    // }
}
