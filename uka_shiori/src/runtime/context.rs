use std::ops::Deref;
use std::path::PathBuf;
use std::sync::Arc;

/// The `ContextData` trait represents shared data that is accessible during the handling of each request.
///
/// Types implementing this trait are used to carry information that is available
/// throughout the lifetime of a SHIORI runtime service. This is typically used
/// to store configuration data, shared resources, or any other kind of data that
/// needs to be accessed across multiple requests.
///
/// `ContextData` provides a way to customize what data is stored and accessed during
/// the processing of a request. For instance, it could be used to hold a connection
/// pool, configuration parameters, or any other kind of shared resource.
///
/// # Examples
///
/// ```rust
/// # use uka_shiori::runtime::ContextData;
/// # use std::path::PathBuf;
/// #
/// pub struct ShioriContext {
///     pub base_dir: PathBuf,
/// }
///
/// impl ShioriContext {
///     pub fn base_dir(&self) -> &PathBuf {
///         &self.base_dir
///     }
/// }
///
/// impl ContextData for ShioriContext {
///     type Error = std::io::Error;
///
///     fn new(path: PathBuf) -> Result<Self, Self::Error> {
///         Ok(Self { base_dir: path })
///     }
/// }
///
/// impl Drop for ShioriContext {
///     fn drop(&mut self) {
///         // ShioriContext does nothing.
///     }
/// }
///
/// let path = PathBuf::from("C:\\ghost\\");
/// let data = ShioriContext::new(path.clone()).unwrap();
/// let context = uka_shiori::runtime::Context::new(data);
///
/// assert_eq!(context.base_dir(), &path);
/// ```
pub trait ContextData: Sized + Send + Sync {
    type Error;

    /// Constructs a new instance of a type implementing `ContextData`.
    ///
    /// The argument `path` is passed as the ghost directory path.
    /// The ghost directory path in the case of `path` is the directory where the DLL files are located.
    fn new(path: PathBuf) -> Result<Self, Self::Error>;
}

/// `Context<T>` is the wrapper for data of type `T` that implements the `ContextData` trait.
///
/// This structure provides a way to carry context-specific data during the handling of a request.
/// It's designed to be used with types implementing the `ContextData` trait, allowing
/// access to shared data during request processing.
pub struct Context<T: ContextData> {
    inner: Arc<T>,
}

impl<T: ContextData> Context<T> {
    pub fn new(value: T) -> Self {
        Self {
            inner: Arc::new(value),
        }
    }
}

impl<T: ContextData> Clone for Context<T> {
    fn clone(&self) -> Self {
        Context {
            inner: self.inner.clone(),
        }
    }
}

impl<T: ContextData> Deref for Context<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl<T: ContextData> From<T> for Context<T> {
    fn from(value: T) -> Self {
        Context::new(value)
    }
}
