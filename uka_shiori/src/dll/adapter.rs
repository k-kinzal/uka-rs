use crate::runtime::{ContextData, Service, Shiori};
use crate::types::v3;
use log::{error, trace};
use std::alloc::System;
use std::ffi::OsString;
use std::future::Future;
use tokio::runtime;
use tokio::runtime::Runtime;
use uka_util::ptr::{OwnedPtr, RawPtr};

/// Adapter is adapted to SHIORI DLL calls.
/// It is an anti-corruption layer and allows Rust-like handling of passed pointers and values.
pub struct Adapter<C, S>
where
    C: ContextData + Send + Sync,
{
    /// The service that handles incoming SHIORI protocol requests.
    shiori: Shiori<C, S>,

    /// The runtime that handles asynchronous tasks.
    runtime: Runtime,
}

impl<C, S, Fut> Adapter<C, S>
where
    C: ContextData<Error = S::Error>,
    S: Service<C, v3::Request, Response = v3::Response, Error = v3::ShioriError, Future = Fut>,
    Fut: Future<Output = Result<S::Response, S::Error>>,
{
    /// load is called when the SHIORI DLL is loaded.
    ///
    /// ```clang
    /// extern "C" __declspec(dllexport) BOOL __cdecl load(HGLOBAL h, long len);
    /// function load(h: hglobal; len: longint): boolean; cdecl;
    /// ```
    ///
    /// # Safety
    ///
    /// This function is safe to use if the following conditions are met:
    ///
    /// 1. `h` must be a valid pointer, meaning it must not be null and it must be properly aligned.
    /// 2. The memory `h` points to must not be shared with other code during the lifetime of the function, meaning that the raw pointer is not aliased.
    /// 3. `len` must accurately represent the number of contiguous bytes that `h` points to.
    ///
    /// When these conditions are upheld, using `load` will not cause undefined behavior.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::path::PathBuf;
    /// # use once_cell::sync::OnceCell;
    /// # use uka_shiori::dll::Adapter;
    /// # use uka_shiori::runtime::{box_handler, BoxAsyncFn, BoxHandlerV3, Context, ContextData, ShioriHandler};
    /// # use uka_shiori::types::v3;
    /// #
    /// struct Data;
    /// impl ContextData for Data {
    ///     type Error = v3::ShioriError;
    ///     fn new(_path: PathBuf) -> Result<Self, Self::Error> {
    ///         Ok(Self)
    ///     }
    /// }
    ///
    /// static SHIORI: OnceCell<Adapter<Data, BoxHandlerV3<Data>>> = OnceCell::new();
    ///     
    /// pub unsafe extern "C" fn load(h: isize, len: usize) -> bool {
    ///     let shiori = SHIORI.get_or_init(|| {
    ///         box_handler(|_ctx: Context<Data>, _req: v3::Request| async {
    ///             unimplemented!("your handler")
    ///         })
    ///         .into()
    ///     });
    ///     shiori.load(h, len)
    /// }
    /// ```
    pub unsafe fn load(&self, h: isize, len: usize) -> bool {
        trace!("call Adapter::load({h}, {len})");
        #[cfg(windows)]
        type Type = u16;
        #[cfg(not(windows))]
        type Type = u8;

        if h == 0 || len == 0 {
            error!("failed load: invalid pointer: {h}, {len}");
            return false;
        }
        let ptr = RawPtr::<[Type]>::from_raw_address_parts(h, len).to_owned::<System>();
        let s = OsString::from_bytes(ptr.as_slice());

        match self.runtime.block_on(self.shiori.load(s.into())) {
            Ok(_) => {
                trace!("call success Adapter::load({h}, {len}) -> true");
                true
            }
            Err(e) => {
                error!("failed load: {e}");
                false
            }
        }
    }

    /// unload is called when the SHIORI DLL is unloaded.
    ///
    /// ```clang
    /// extern "C" __declspec(dllexport) BOOL __cdecl unload();
    /// function unload: boolean; cdecl;
    /// ```
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::path::PathBuf;
    /// # use once_cell::sync::OnceCell;
    /// # use uka_shiori::dll::Adapter;
    /// # use uka_shiori::runtime::{box_handler, BoxAsyncFn, BoxHandlerV3, Context, ContextData, ShioriHandler};
    /// # use uka_shiori::types::v3;
    /// #
    /// struct Data;
    /// impl ContextData for Data {
    ///     type Error = v3::ShioriError;
    ///     fn new(_path: PathBuf) -> Result<Self, Self::Error> {
    ///         Ok(Self)
    ///     }
    /// }
    ///
    /// static SHIORI: OnceCell<Adapter<Data, BoxHandlerV3<Data>>> = OnceCell::new();
    ///
    /// pub unsafe extern "C" fn unload() -> bool {
    ///     match SHIORI.get() {
    ///         Some(shiori) => shiori.unload(),
    ///         None => false,
    ///     }
    /// }
    /// ```
    pub fn unload(&self) -> bool {
        trace!("call Adapter::unload()");
        match self.runtime.block_on(self.shiori.unload()) {
            Ok(_) => {
                trace!("call success Adapter::unload() -> true");
                true
            }
            Err(e) => {
                error!("failed unload: {e}");
                false
            }
        }
    }

    /// request is called when the SHIORI DLL receives a request.
    ///
    /// ```clang
    /// extern "C" __declspec(dllexport) HGLOBAL __cdecl request(HGLOBAL h, long *len);
    /// function request(h: hglobal; var len: longint): hglobal; cdecl; export;
    /// ```
    ///
    /// # Safety
    ///
    /// This function is safe to use if the following conditions are met:
    ///
    /// 1. `h` must be a valid pointer, meaning it must not be null and it must be properly aligned.
    /// 2. `len` must be a valid pointer to a memory region containing a `usize` and it must not be null.
    /// 3. The memory `h` points to must not be shared with other code during the lifetime of the function, meaning that the raw pointer is not aliased.
    ///
    /// When these conditions are upheld, using `request` will not cause undefined behavior.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::path::PathBuf;
    /// # use once_cell::sync::OnceCell;
    /// # use uka_shiori::dll::Adapter;
    /// # use uka_shiori::runtime::{box_handler, BoxAsyncFn, BoxHandlerV3, Context, ContextData, ShioriHandler};
    /// # use uka_shiori::types::v3;
    /// #
    /// struct Data;
    /// impl ContextData for Data {
    ///     type Error = v3::ShioriError;
    ///     fn new(_path: PathBuf) -> Result<Self, Self::Error> {
    ///         Ok(Self)
    ///     }
    /// }
    ///
    /// static SHIORI: OnceCell<Adapter<Data, BoxHandlerV3<Data>>> = OnceCell::new();
    ///  
    /// pub unsafe extern "C" fn request(h: isize, len: *mut usize) -> isize {
    ///     match SHIORI.get() {
    ///         Some(shiori) => shiori.request(h, len),
    ///         None => 0,
    ///     }
    /// }
    /// ```
    pub unsafe fn request(&self, h: isize, len: *mut usize) -> isize {
        use v3::IntoResponse;

        if h == 0 || len.is_null() {
            error!("failed request: invalid pointer: {h}, {len:?}");
            return 0;
        }
        trace!("call Adapter::request({h}, {})", *len);

        let len = RawPtr::<usize>::from(len);
        let ptr = RawPtr::<[u8]>::from_raw_address_parts(h, *len.as_ref()).to_owned::<System>();
        let resp = match v3::Request::parse(ptr.as_slice()) {
            Ok(request) => self.runtime.block_on(self.shiori.request(request)),
            Err(e) => {
                error!("failed request: {e}");
                v3::ShioriError::from(e)
                    .with_status_code(v3::StatusCode::BAD_REQUEST)
                    .into_response()
            }
        };

        let bytes = resp.as_bytes();
        len.as_mut_ptr().write(bytes.len());
        let h = OwnedPtr::from_vec(bytes).into_raw_slice().as_ptr() as isize;

        trace!(
            "call success Adapter::request({h}, {}) -> {h}",
            *len.as_ref()
        );

        h
    }
}

impl<C, S> From<Shiori<C, S>> for Adapter<C, S>
where
    C: ContextData<Error = S::Error>,
    S: Service<C, v3::Request, Response = v3::Response, Error = v3::ShioriError>,
{
    fn from(value: Shiori<C, S>) -> Self {
        Adapter {
            shiori: value,
            runtime: runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("failed to create tokio runtime"),
        }
    }
}

impl<C, S> From<S> for Adapter<C, S>
where
    C: ContextData<Error = S::Error>,
    S: Service<C, v3::Request, Response = v3::Response, Error = v3::ShioriError>,
{
    fn from(value: S) -> Self {
        let shiori = Shiori::from(value);
        Adapter {
            shiori,
            runtime: runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("failed to create tokio runtime"),
        }
    }
}

trait BytesExt {
    type Type;
    fn from_bytes(bytes: &[Self::Type]) -> Self;
    fn to_vec(&self) -> Vec<Self::Type>;
}

#[cfg(not(windows))]
impl BytesExt for OsString {
    type Type = u8;

    fn from_bytes(bytes: &[u8]) -> Self {
        <std::ffi::OsStr as std::os::unix::ffi::OsStrExt>::from_bytes(bytes).to_os_string()
    }

    fn to_vec(&self) -> Vec<u8> {
        <std::ffi::OsStr as std::os::unix::ffi::OsStrExt>::as_bytes(self.as_os_str()).to_vec()
    }
}

#[cfg(windows)]
impl BytesExt for OsString {
    type Type = u16;

    fn from_bytes(bytes: &[u16]) -> Self {
        <std::ffi::OsString as std::os::windows::ffi::OsStringExt>::from_wide(bytes)
    }

    fn to_vec(&self) -> Vec<u16> {
        use std::os::windows::ffi::OsStrExt;
        self.as_os_str().encode_wide().collect::<Vec<u16>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::{handler, Context};
    use std::ffi::c_void;
    use std::fs;
    use std::mem::ManuallyDrop;
    use std::path::PathBuf;
    use std::ptr::null;

    struct Data;
    impl ContextData for Data {
        type Error = v3::ShioriError;
        fn new(path: PathBuf) -> Result<Self, Self::Error> {
            let result = fs::read_dir(path);
            assert!(result.is_ok(), "{}", result.unwrap_err());

            Ok(Self)
        }
    }

    #[test]
    fn test_adapter_load() {
        let adapter = Adapter::from(handler(|_ctx: Context<Data>, _req: v3::Request| async {
            unimplemented!()
        }));

        let path = std::env::temp_dir();
        let path = path.join("マルチバイトディレクトリ");
        let _ = fs::create_dir(path.clone());

        let bytes = ManuallyDrop::new(path.into_os_string().to_vec());
        let res = unsafe { adapter.load(bytes.as_ptr() as isize, bytes.len()) };
        assert!(res);
    }

    #[test]
    fn test_adapter_load_with_invalid_pointer() {
        let adapter = Adapter::from(handler(|_ctx: Context<Data>, _req: v3::Request| async {
            unimplemented!()
        }));

        let path = std::env::temp_dir();
        let path = path.join("マルチバイトディレクトリ");
        let _ = fs::create_dir(path);

        let res = unsafe { adapter.load(null::<c_void>() as isize, 0) };
        assert!(!res);
    }

    #[test]
    fn test_adapter_unload() {
        let adapter = Adapter::from(handler(|_ctx: Context<Data>, _req: v3::Request| async {
            unimplemented!()
        }));

        let path = std::env::temp_dir();
        let path = path.join("マルチバイトディレクトリ");
        let _ = fs::create_dir(path.clone());

        let bytes = ManuallyDrop::new(path.into_os_string().to_vec());
        let res = unsafe { adapter.load(bytes.as_ptr() as isize, bytes.len()) };
        assert!(res);

        let res = adapter.unload();
        assert!(res);
    }

    #[test]
    fn test_adapter_request() {
        let adapter = Adapter::from(handler(
            |_ctx: Context<Data>, req: v3::Request| async move {
                assert_eq!(req.method(), v3::Method::GET);
                assert_eq!(req.version(), v3::Version::SHIORI_30);

                Ok(v3::Response::builder()
                    .status_code(v3::StatusCode::OK)
                    .version(v3::Version::SHIORI_30)
                    .build()
                    .expect("failed to build response"))
            },
        ));

        let path = std::env::temp_dir();
        let path = path.join("マルチバイトディレクトリ");
        let _ = fs::create_dir(path.clone());

        let bytes = ManuallyDrop::new(path.into_os_string().to_vec());
        let res = unsafe { adapter.load(bytes.as_ptr() as isize, bytes.len()) };
        assert!(res);

        let req = v3::Request::builder()
            .method(v3::Method::GET)
            .version(v3::Version::SHIORI_30)
            .build()
            .expect("failed to build request");
        let bytes = ManuallyDrop::new(req.as_bytes());
        let len = bytes.len();
        let h =
            unsafe { adapter.request(bytes.as_ptr() as isize, &len as *const usize as *mut usize) };

        let ptr = unsafe { RawPtr::<[u8]>::from_raw_address_parts(h, len) };
        let bytes = unsafe { ptr.as_slice() };
        let res = v3::Response::parse(bytes).expect("failed to parse response");
        assert_eq!(res.version(), v3::Version::SHIORI_30);
        assert_eq!(res.status_code(), v3::StatusCode::OK);
    }

    #[test]
    fn test_adapter_request_with_invalid_pointer() {
        let adapter = Adapter::from(handler(|_ctx: Context<Data>, _req: v3::Request| async {
            unimplemented!()
        }));

        let path = std::env::temp_dir();
        let path = path.join("マルチバイトディレクトリ");
        let _ = fs::create_dir(path.clone());

        let bytes = ManuallyDrop::new(path.into_os_string().to_vec());
        let res = unsafe { adapter.load(bytes.as_ptr() as isize, bytes.len()) };
        assert!(res);

        let h =
            unsafe { adapter.request(null::<c_void>() as isize, null::<usize>() as *mut usize) };
        assert_eq!(h, 0);
    }

    #[test]
    fn test_adapter_request_with_invalid_request() {
        let adapter = Adapter::from(handler(|_ctx: Context<Data>, _req: v3::Request| async {
            unimplemented!()
        }));

        let path = std::env::temp_dir();
        let path = path.join("マルチバイトディレクトリ");
        let _ = fs::create_dir(path.clone());

        let bytes = ManuallyDrop::new(path.into_os_string().to_vec());
        let res = unsafe { adapter.load(bytes.as_ptr() as isize, bytes.len()) };
        assert!(res);

        let bytes = ManuallyDrop::new(b"invalid".to_vec());
        let len = bytes.len();
        let h =
            unsafe { adapter.request(bytes.as_ptr() as isize, &len as *const usize as *mut usize) };

        let ptr = unsafe { RawPtr::<[u8]>::from_raw_address_parts(h, len) };
        let bytes = unsafe { ptr.as_slice() };
        let res = v3::Response::parse(bytes).expect("failed to parse response");
        assert_eq!(res.version(), v3::Version::SHIORI_30);
        assert_eq!(res.status_code(), v3::StatusCode::BAD_REQUEST);
    }
}
