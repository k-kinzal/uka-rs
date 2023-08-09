use crate::runtime::context::{Context, ContextData};
use crate::types::v3;
use std::future::Future;
use std::pin::Pin;

/// `Service<C, R>` is the trait defining an interface for implementing SHIORI runtime services.
///
/// Services implementing this trait should be capable of processing incoming requests
/// according to the SHIORI protocol. `C` represents context-specific data, while `R`
/// denotes the request type.
///
/// `Response`, `Error` and `Future` associated types define the service's response,
/// potential error, and the future of the result respectively.
pub trait Service<C, R>
where
    C: ContextData,
{
    type Response;
    type Error;
    type Future: Future<Output = Result<Self::Response, Self::Error>>;

    /// Process an incoming SHIORI protocol request.
    ///
    /// Given the runtime context and an incoming request, this method should
    /// provide the logic for handling the request and produce a future
    /// that results in either a service response or an error.
    fn call(&self, context: Context<C>, request: R) -> Self::Future;
}

/// `ShioriHandler<F>` is a handler for SHIORI runtime services that implements `Service<C, Request>`.
///
/// This struct provides a convenient way to wrap a function that can process requests
/// into a handler, which can be used by a server or other components to handle requests.
pub struct ShioriHandler<F> {
    /// The function responsible for handling incoming requests.
    ///
    /// This function should be capable of processing SHIORI protocol requests,
    /// and it should return a future that results in either a service response or an error.
    handle: F,
}

impl<Ctx, Req, Res, Err, F, Fut> Service<Ctx, Req> for ShioriHandler<F>
where
    Ctx: ContextData,
    F: Fn(Context<Ctx>, Req) -> Fut,
    Fut: Future<Output = Result<Res, Err>>,
{
    type Response = Res;
    type Error = Err;
    type Future = Fut;

    fn call(&self, context: Context<Ctx>, request: Req) -> Self::Future {
        (self.handle)(context, request)
    }
}

/// This function provides a convenient way to construct a `ShioriHandler` from a function.
///
/// The input function `f` should take a runtime context and a SHIORI protocol request as
/// arguments, and return a future that results in either a service response or an error.
///
/// The created handler can be used by a server or other components to handle incoming
/// SHIORI protocol requests.
pub fn handler<Ctx, Req, Res, Err, F, Fut>(f: F) -> ShioriHandler<F>
where
    Ctx: ContextData,
    F: Fn(Context<Ctx>, Req) -> Fut,
    Fut: Future<Output = Result<Res, Err>>,
{
    ShioriHandler { handle: f }
}

pub type BoxAsyncFn<Ctx, Req, Res, Err> =
    Box<dyn Fn(Context<Ctx>, Req) -> Pin<Box<dyn Future<Output = Result<Res, Err>>>> + Send + Sync>;

pub type BoxHandler<Ctx, Req, Res, Err> = ShioriHandler<BoxAsyncFn<Ctx, Req, Res, Err>>;

pub type BoxHandlerV3<Ctx> = BoxHandler<Ctx, v3::Request, v3::Response, v3::ShioriError>;

/// Provides a convenient way to construct a `ShioriHandler` from a function, and wraps the function in a `Box`.
///
/// This is used over `handler` when you need to store the handler in a `OnceCell` or similar structures
pub fn box_handler<Ctx, Req, Res, Err, F, Fut>(
    f: F,
) -> ShioriHandler<BoxAsyncFn<Ctx, Req, Res, Err>>
where
    Ctx: ContextData,
    F: Fn(Context<Ctx>, Req) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<Res, Err>> + 'static,
{
    ShioriHandler {
        handle: Box::new(move |ctx, req| Box::pin(f(ctx, req))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::v3;
    use std::path::PathBuf;

    struct Data;
    impl ContextData for Data {
        type Error = ();

        fn new(_path: PathBuf) -> Result<Self, Self::Error> {
            Ok(Self)
        }
    }

    #[tokio::test]
    async fn test_handler_with_closure() -> Result<(), v3::ShioriError> {
        let handler = handler(|_ctx: Context<Data>, _req: v3::Request| async {
            v3::Response::builder()
                .version(v3::Version::SHIORI_30)
                .status_code(v3::StatusCode::OK)
                .build()
                .map_err(v3::ShioriError::from)
        });

        let ctx = Context::from(Data);
        let req = v3::Request::builder()
            .version(v3::Version::SHIORI_30)
            .method(v3::Method::GET)
            .header(v3::HeaderName::CHARSET, v3::Charset::UTF8.to_string())
            .build()?;
        let resp = handler.call(ctx, req).await?;
        assert_eq!(resp.version(), v3::Version::SHIORI_30);
        assert_eq!(resp.status_code(), v3::StatusCode::OK);

        Ok(())
    }

    #[tokio::test]
    async fn test_handler_with_fn() -> Result<(), v3::ShioriError> {
        #[allow(clippy::manual_async_fn)]
        fn handle(
            _ctx: Context<Data>,
            _req: v3::Request,
        ) -> impl Future<Output = Result<v3::Response, v3::ShioriError>> {
            async {
                v3::Response::builder()
                    .version(v3::Version::SHIORI_30)
                    .status_code(v3::StatusCode::OK)
                    .build()
                    .map_err(v3::ShioriError::from)
            }
        }

        let handler = handler(handle);

        let ctx = Context::from(Data);
        let req = v3::Request::builder()
            .version(v3::Version::SHIORI_30)
            .method(v3::Method::GET)
            .header(v3::HeaderName::CHARSET, v3::Charset::UTF8.to_string())
            .build()?;
        let resp = handler.call(ctx, req).await?;
        assert_eq!(resp.version(), v3::Version::SHIORI_30);
        assert_eq!(resp.status_code(), v3::StatusCode::OK);

        Ok(())
    }

    #[tokio::test]
    async fn test_handler_with_async_fn() -> Result<(), v3::ShioriError> {
        async fn handle(
            _ctx: Context<Data>,
            _req: v3::Request,
        ) -> Result<v3::Response, v3::ShioriError> {
            v3::Response::builder()
                .version(v3::Version::SHIORI_30)
                .status_code(v3::StatusCode::OK)
                .build()
                .map_err(v3::ShioriError::from)
        }

        let handler = handler(handle);

        let ctx = Context::from(Data);
        let req = v3::Request::builder()
            .version(v3::Version::SHIORI_30)
            .method(v3::Method::GET)
            .header(v3::HeaderName::CHARSET, v3::Charset::UTF8.to_string())
            .build()?;
        let resp = handler.call(ctx, req).await?;
        assert_eq!(resp.version(), v3::Version::SHIORI_30);
        assert_eq!(resp.status_code(), v3::StatusCode::OK);

        Ok(())
    }

    #[tokio::test]
    async fn test_box_handler_with_closure() -> Result<(), v3::ShioriError> {
        let handler = box_handler(|_ctx: Context<Data>, _req: v3::Request| async {
            v3::Response::builder()
                .version(v3::Version::SHIORI_30)
                .status_code(v3::StatusCode::OK)
                .build()
                .map_err(v3::ShioriError::from)
        });

        let ctx = Context::from(Data);
        let req = v3::Request::builder()
            .version(v3::Version::SHIORI_30)
            .method(v3::Method::GET)
            .header(v3::HeaderName::CHARSET, v3::Charset::UTF8.to_string())
            .build()?;
        let resp = handler.call(ctx, req).await?;
        assert_eq!(resp.version(), v3::Version::SHIORI_30);
        assert_eq!(resp.status_code(), v3::StatusCode::OK);

        Ok(())
    }

    #[tokio::test]
    async fn test_box_handler_with_fn() -> Result<(), v3::ShioriError> {
        #[allow(clippy::manual_async_fn)]
        fn handle(
            _ctx: Context<Data>,
            _req: v3::Request,
        ) -> impl Future<Output = Result<v3::Response, v3::ShioriError>> {
            async {
                v3::Response::builder()
                    .version(v3::Version::SHIORI_30)
                    .status_code(v3::StatusCode::OK)
                    .build()
                    .map_err(v3::ShioriError::from)
            }
        }

        let handler = box_handler(handle);

        let ctx = Context::from(Data);
        let req = v3::Request::builder()
            .version(v3::Version::SHIORI_30)
            .method(v3::Method::GET)
            .header(v3::HeaderName::CHARSET, v3::Charset::UTF8.to_string())
            .build()?;
        let resp = handler.call(ctx, req).await?;
        assert_eq!(resp.version(), v3::Version::SHIORI_30);
        assert_eq!(resp.status_code(), v3::StatusCode::OK);

        Ok(())
    }

    #[tokio::test]
    async fn test_box_handler_with_async_fn() -> Result<(), v3::ShioriError> {
        async fn handle(
            _ctx: Context<Data>,
            _req: v3::Request,
        ) -> Result<v3::Response, v3::ShioriError> {
            v3::Response::builder()
                .version(v3::Version::SHIORI_30)
                .status_code(v3::StatusCode::OK)
                .build()
                .map_err(v3::ShioriError::from)
        }

        let handler = box_handler(handle);

        let ctx = Context::from(Data);
        let req = v3::Request::builder()
            .version(v3::Version::SHIORI_30)
            .method(v3::Method::GET)
            .header(v3::HeaderName::CHARSET, v3::Charset::UTF8.to_string())
            .build()?;
        let resp = handler.call(ctx, req).await?;
        assert_eq!(resp.version(), v3::Version::SHIORI_30);
        assert_eq!(resp.status_code(), v3::StatusCode::OK);

        Ok(())
    }
}
