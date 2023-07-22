use crate::runtime::context::{Context, ContextData};
use crate::runtime::error::ShioriError;
use crate::runtime::service::Service;
use crate::types::{Request, Response};
use std::future::Future;
use std::ops::Deref;
use std::path::PathBuf;
use tokio::sync::RwLock;

/// `Shiori<C, S>` represents the SHIORI runtime.
///
/// The runtime is responsible for managing and executing services that handle
/// SHIORI protocol requests. `C` represents context-specific data and `S` represents the service.
pub struct Shiori<C, S>
where
    C: ContextData<Error = ShioriError>,
    S: Service<C, Request, Response = Response, Error = ShioriError>,
{
    /// Shared context data, which is available to the service during the processing of requests.
    context: RwLock<Option<Context<C>>>,

    /// The service that handles incoming SHIORI protocol requests.
    service: S,
}

impl<C, S, Fut> Shiori<C, S>
where
    C: ContextData<Error = ShioriError>,
    S: Service<C, Request, Response = Response, Error = ShioriError, Future = Fut>,
    Fut: Future<Output = Result<Response, ShioriError>>,
{
    /// Initialize the SHIORI runtime by loading context data from the provided path.
    ///
    /// In accordance with SHIORI protocol, if there are any associated data files, they should be
    /// located at this path. This method must be called before processing any requests.
    pub async fn load(&self, path: PathBuf) -> Result<(), ShioriError> {
        let mut context = self.context.write().await;
        match context.deref() {
            Some(_) => Err(ShioriError::new("context already loaded")),
            None => {
                let data = C::new(path)?;
                context.replace(data.into());

                Ok(())
            }
        }
    }

    /// Unload the context, removing all the context data.
    ///
    /// After this method is called, no requests can be processed until the context is loaded again.
    pub async fn unload(&self) -> Result<(), ShioriError> {
        let mut context = self.context.write().await;
        match context.deref() {
            Some(_) => {
                context.take();
                Ok(())
            }
            None => Err(ShioriError::new("context not loaded")),
        }
    }

    /// Process a SHIORI protocol request.
    ///
    /// This method accepts a request, passes it to the service for processing, and returns the
    /// service's response. The context must be loaded before this method is called.
    pub async fn request(&self, request: Request) -> Result<Response, ShioriError> {
        let ctx = self.context.read().await;
        match ctx.deref() {
            Some(ctx) => self.service.call(ctx.clone(), request).await,
            None => Err(ShioriError::new("context not loaded")),
        }
    }
}

impl<C, S> From<S> for Shiori<C, S>
where
    C: ContextData<Error = ShioriError>,
    S: Service<C, Request, Response = Response, Error = ShioriError>,
{
    fn from(value: S) -> Self {
        Self {
            context: RwLock::new(None),
            service: value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::handler;
    use crate::types::{v3, RequestExt, ResponseExt};

    struct ShioriContext {
        path: PathBuf,
    }
    impl ShioriContext {
        fn path(&self) -> &PathBuf {
            &self.path
        }
    }
    impl ContextData for ShioriContext {
        type Error = ShioriError;

        fn new(path: PathBuf) -> Result<Self, ShioriError> {
            Ok(Self { path })
        }
    }

    #[tokio::test]
    async fn test_load() -> Result<(), ShioriError> {
        let shiori = Shiori::from(handler(
            |_ctx: Context<ShioriContext>, _req: Request| async { unimplemented!() },
        ));

        let result = shiori.load(PathBuf::from(".")).await;
        assert!(result.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn test_load_failed_call_after_load() -> Result<(), ShioriError> {
        let shiori = Shiori::from(handler(
            |_ctx: Context<ShioriContext>, _req: Request| async { unimplemented!() },
        ));

        let result = shiori.load(PathBuf::from(".")).await;
        assert!(result.is_ok());

        let result = shiori.load(PathBuf::from(".")).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "context already loaded");

        Ok(())
    }

    #[tokio::test]
    async fn test_unload() -> Result<(), ShioriError> {
        let shiori = Shiori::from(handler(
            |_ctx: Context<ShioriContext>, _req: Request| async { unimplemented!() },
        ));

        shiori.load(PathBuf::from(".")).await?;

        let result = shiori.unload().await;
        assert!(result.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn test_unload_failed_not_call_load() -> Result<(), ShioriError> {
        let shiori = Shiori::from(handler(
            |_ctx: Context<ShioriContext>, _req: Request| async { unimplemented!() },
        ));

        let result = shiori.unload().await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "context not loaded");

        Ok(())
    }

    #[tokio::test]
    async fn test_request() -> Result<(), ShioriError> {
        let shiori = Shiori::from(handler(
            |ctx: Context<ShioriContext>, req: Request| async move {
                assert_eq!(ctx.path(), &PathBuf::from("."));
                matches!(req, Request::V3(ref r) if r.version() == v3::Version::SHIORI_30);
                matches!(req, Request::V3(ref r) if r.method() == v3::Method::GET);

                let resp = Response::builder(v3::Version::SHIORI_30)
                    .status_code(v3::StatusCode::OK)
                    .build()?;
                Ok(resp.into())
            },
        ));

        shiori.load(PathBuf::from(".")).await?;

        let request = Request::builder(v3::Version::SHIORI_30)
            .method(v3::Method::GET)
            .charset(v3::Charset::UTF8)
            .build()?;
        let response = shiori.request(request.into()).await?;

        matches!(response, Response::V3(ref r) if r.version() == v3::Version::SHIORI_30);
        matches!(response, Response::V3(ref r) if r.status_code() == v3::StatusCode::OK);

        Ok(())
    }

    #[tokio::test]
    async fn test_request_failed_not_call_load() -> Result<(), ShioriError> {
        let shiori = Shiori::from(handler(
            |_ctx: Context<ShioriContext>, _req: Request| async { unimplemented!() },
        ));

        let request = Request::builder(v3::Version::SHIORI_30)
            .method(v3::Method::GET)
            .charset(v3::Charset::UTF8)
            .build()?;
        let result = shiori.request(request.into()).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "context not loaded");

        Ok(())
    }

    #[tokio::test]
    async fn test_request_failed_handle_error() -> Result<(), ShioriError> {
        let shiori = Shiori::from(handler(
            |_ctx: Context<ShioriContext>, _req: Request| async {
                Err(ShioriError::new("handler error"))
            },
        ));

        shiori.load(PathBuf::from(".")).await?;

        let request = Request::builder(v3::Version::SHIORI_30)
            .method(v3::Method::GET)
            .charset(v3::Charset::UTF8)
            .build()?;
        let result = shiori.request(request.into()).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "handler error");

        Ok(())
    }
}
