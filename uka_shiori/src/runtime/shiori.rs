use crate::runtime::context::{Context, ContextData};
use crate::runtime::service::Service;
use crate::types::v3;
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
    C: ContextData,
{
    /// Shared context data, which is available to the service during the processing of requests.
    context: RwLock<Option<Context<C>>>,

    /// The service that handles incoming SHIORI protocol requests.
    service: S,
}

impl<C, S, Fut> Shiori<C, S>
where
    C: ContextData<Error = S::Error>,
    S: Service<C, v3::Request, Response = v3::Response, Error = v3::ShioriError, Future = Fut>,
    Fut: Future<Output = Result<S::Response, S::Error>>,
{
    /// Initialize the SHIORI runtime by loading context data from the provided path.
    ///
    /// In accordance with SHIORI protocol, if there are any associated data files, they should be
    /// located at this path. This method must be called before processing any requests.
    pub async fn load(&self, path: PathBuf) -> Result<(), S::Error> {
        let mut context = self.context.write().await;
        match context.deref() {
            Some(_) => Err(S::Error::new("context already loaded")),
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
    pub async fn unload(&self) -> Result<(), S::Error> {
        let mut context = self.context.write().await;
        match context.deref() {
            Some(_) => {
                context.take();
                Ok(())
            }
            None => Err(S::Error::new("context not loaded")),
        }
    }

    /// Process a SHIORI protocol request.
    ///
    /// This method accepts a request, passes it to the service for processing, and returns the
    /// service's response. The context must be loaded before this method is called.
    pub async fn request(&self, request: v3::Request) -> S::Response {
        use v3::IntoResponse;

        let ctx = self.context.read().await;
        let result = match ctx.deref() {
            Some(ctx) => self.service.call(ctx.clone(), request).await,
            None => Err(v3::ShioriError::new("context not loaded")),
        };
        match result {
            Ok(resp) => resp,
            Err(err) => err.into_response(),
        }
    }
}

impl<C, S> From<S> for Shiori<C, S>
where
    C: ContextData<Error = v3::ShioriError>,
    S: Service<C, v3::Request, Response = v3::Response, Error = v3::ShioriError>,
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
    use crate::types::v3;

    struct ShioriContext {
        path: PathBuf,
    }
    impl ShioriContext {
        fn path(&self) -> &PathBuf {
            &self.path
        }
    }
    impl ContextData for ShioriContext {
        type Error = v3::ShioriError;

        fn new(path: PathBuf) -> Result<Self, v3::ShioriError> {
            Ok(Self { path })
        }
    }

    #[tokio::test]
    async fn test_load() -> Result<(), v3::ShioriError> {
        let shiori = Shiori::from(handler(
            |_ctx: Context<ShioriContext>, _req: v3::Request| async { unimplemented!() },
        ));

        let result = shiori.load(PathBuf::from(".")).await;
        assert!(result.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn test_load_failed_call_after_load() -> Result<(), v3::ShioriError> {
        let shiori = Shiori::from(handler(
            |_ctx: Context<ShioriContext>, _req: v3::Request| async { unimplemented!() },
        ));

        let result = shiori.load(PathBuf::from(".")).await;
        assert!(result.is_ok());

        let result = shiori.load(PathBuf::from(".")).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "context already loaded");

        Ok(())
    }

    #[tokio::test]
    async fn test_unload() -> Result<(), v3::ShioriError> {
        let shiori = Shiori::from(handler(
            |_ctx: Context<ShioriContext>, _req: v3::Request| async { unimplemented!() },
        ));

        shiori.load(PathBuf::from(".")).await?;

        let result = shiori.unload().await;
        assert!(result.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn test_unload_failed_not_call_load() -> Result<(), v3::ShioriError> {
        let shiori = Shiori::from(handler(
            |_ctx: Context<ShioriContext>, _req: v3::Request| async { unimplemented!() },
        ));

        let result = shiori.unload().await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "context not loaded");

        Ok(())
    }

    #[tokio::test]
    async fn test_request() -> Result<(), v3::ShioriError> {
        let shiori = Shiori::from(handler(
            |ctx: Context<ShioriContext>, req: v3::Request| async move {
                assert_eq!(ctx.path(), &PathBuf::from("."));
                assert_eq!(req.version(), v3::Version::SHIORI_30);
                assert_eq!(req.method(), v3::Method::GET);

                v3::Response::builder()
                    .version(v3::Version::SHIORI_30)
                    .status_code(v3::StatusCode::OK)
                    .build()
                    .map_err(v3::ShioriError::from)
            },
        ));

        shiori.load(PathBuf::from(".")).await?;

        let req = v3::Request::builder()
            .version(v3::Version::SHIORI_30)
            .method(v3::Method::GET)
            .charset(v3::Charset::UTF8)
            .build()?;
        let resp = shiori.request(req).await;

        assert_eq!(resp.version(), v3::Version::SHIORI_30);
        assert_eq!(resp.status_code(), v3::StatusCode::OK);

        Ok(())
    }

    #[tokio::test]
    async fn test_request_failed_not_call_load() -> Result<(), v3::ShioriError> {
        let shiori = Shiori::from(handler(
            |_ctx: Context<ShioriContext>, _req: v3::Request| async { unimplemented!() },
        ));

        let req = v3::Request::builder()
            .version(v3::Version::SHIORI_30)
            .method(v3::Method::GET)
            .charset(v3::Charset::UTF8)
            .build()?;
        let resp = shiori.request(req).await;
        assert_eq!(resp.version(), v3::Version::SHIORI_30);
        assert_eq!(resp.status_code(), v3::StatusCode::INTERNAL_SERVER_ERROR);

        Ok(())
    }

    #[tokio::test]
    async fn test_request_failed_handle_error() -> Result<(), v3::ShioriError> {
        let shiori = Shiori::from(handler(
            |_ctx: Context<ShioriContext>, _req: v3::Request| async {
                Err(v3::ShioriError::new("handler error")
                    .with_status_code(v3::StatusCode::BAD_REQUEST))
            },
        ));

        shiori.load(PathBuf::from(".")).await?;

        let req = v3::Request::builder()
            .version(v3::Version::SHIORI_30)
            .method(v3::Method::GET)
            .charset(v3::Charset::UTF8)
            .build()?;
        let resp = shiori.request(req).await;
        assert_eq!(resp.version(), v3::Version::SHIORI_30);
        assert_eq!(resp.status_code(), v3::StatusCode::BAD_REQUEST);

        Ok(())
    }
}
