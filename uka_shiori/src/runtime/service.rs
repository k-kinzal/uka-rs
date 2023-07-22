use crate::runtime::context::{Context, ContextData};
use crate::runtime::error::ShioriError;
use crate::types::{Request, Response};
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

impl<C, F, Fut> Service<C, Request> for ShioriHandler<F>
where
    C: ContextData,
    F: Fn(Context<C>, Request) -> Fut,
    Fut: Future<Output = Result<Response, ShioriError>>,
{
    type Response = Response;
    type Error = ShioriError;
    type Future = Fut;

    fn call(&self, context: Context<C>, request: Request) -> Self::Future {
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
pub fn handler<C, F, Fut>(f: F) -> ShioriHandler<F>
where
    C: ContextData,
    F: Fn(Context<C>, Request) -> Fut,
    Fut: Future<Output = Result<Response, ShioriError>>,
{
    ShioriHandler { handle: f }
}

pub type BoxAsyncFn<C> = Box<
    dyn Fn(Context<C>, Request) -> Pin<Box<dyn Future<Output = Result<Response, ShioriError>>>>,
>;

/// Provides a convenient way to construct a `ShioriHandler` from a function, and wraps the function in a `Box`.
///
/// This is used over `handler` when you need to store the handler in a `OnceCell` or similar structures
pub fn box_handler<C, F, Fut>(f: F) -> ShioriHandler<BoxAsyncFn<C>>
where
    C: ContextData,
    F: Fn(Context<C>, Request) -> Fut + 'static,
    Fut: Future<Output = Result<Response, ShioriError>> + 'static,
{
    ShioriHandler {
        handle: Box::new(move |ctx, req| Box::pin(f(ctx, req))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{v3, RequestExt, ResponseExt};
    use std::path::PathBuf;

    struct Data;
    impl ContextData for Data {
        type Error = ();

        fn new(_path: PathBuf) -> Result<Self, Self::Error> {
            Ok(Self)
        }
    }

    #[tokio::test]
    async fn test_handler_with_closure() -> Result<(), ShioriError> {
        let handler = handler(|_ctx: Context<Data>, _req: Request| async {
            let resp = Response::builder(v3::Version::SHIORI_30)
                .status_code(v3::StatusCode::OK)
                .build()?;
            Ok(resp.into())
        });

        let ctx = Context::from(Data);
        let req = Request::builder(v3::Version::SHIORI_30)
            .method(v3::Method::GET)
            .header(v3::HeaderName::CHARSET, v3::Charset::UTF8.to_string())
            .build()?;
        let resp = handler.call(ctx, req.into()).await?;
        match resp {
            Response::V3(r) => {
                assert_eq!(r.version(), v3::Version::SHIORI_30);
                assert_eq!(r.status_code(), v3::StatusCode::OK);
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_handler_with_fn() -> Result<(), ShioriError> {
        #[allow(clippy::manual_async_fn)]
        fn handle(
            _ctx: Context<Data>,
            _req: Request,
        ) -> impl Future<Output = Result<Response, ShioriError>> {
            async {
                let resp = Response::builder(v3::Version::SHIORI_30)
                    .status_code(v3::StatusCode::OK)
                    .build()?;
                Ok(resp.into())
            }
        }

        let handler = handler(handle);

        let ctx = Context::from(Data);
        let req = Request::builder(v3::Version::SHIORI_30)
            .method(v3::Method::GET)
            .header(v3::HeaderName::CHARSET, v3::Charset::UTF8.to_string())
            .build()?;
        let resp = handler.call(ctx, req.into()).await?;
        match resp {
            Response::V3(r) => {
                assert_eq!(r.version(), v3::Version::SHIORI_30);
                assert_eq!(r.status_code(), v3::StatusCode::OK);
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_handler_with_async_fn() -> Result<(), ShioriError> {
        async fn handle(_ctx: Context<Data>, _req: Request) -> Result<Response, ShioriError> {
            let resp = Response::builder(v3::Version::SHIORI_30)
                .status_code(v3::StatusCode::OK)
                .build()?;
            Ok(resp.into())
        }

        let handler = handler(handle);

        let ctx = Context::from(Data);
        let req = Request::builder(v3::Version::SHIORI_30)
            .method(v3::Method::GET)
            .header(v3::HeaderName::CHARSET, v3::Charset::UTF8.to_string())
            .build()?;
        let resp = handler.call(ctx, req.into()).await?;
        match resp {
            Response::V3(r) => {
                assert_eq!(r.version(), v3::Version::SHIORI_30);
                assert_eq!(r.status_code(), v3::StatusCode::OK);
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_box_handler_with_closure() -> Result<(), ShioriError> {
        let handler = box_handler(|_ctx: Context<Data>, _req: Request| async {
            let resp = Response::builder(v3::Version::SHIORI_30)
                .status_code(v3::StatusCode::OK)
                .build()?;
            Ok(resp.into())
        });

        let ctx = Context::from(Data);
        let req = Request::builder(v3::Version::SHIORI_30)
            .method(v3::Method::GET)
            .header(v3::HeaderName::CHARSET, v3::Charset::UTF8.to_string())
            .build()?;
        let resp = handler.call(ctx, req.into()).await?;
        match resp {
            Response::V3(r) => {
                assert_eq!(r.version(), v3::Version::SHIORI_30);
                assert_eq!(r.status_code(), v3::StatusCode::OK);
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_box_handler_with_fn() -> Result<(), ShioriError> {
        #[allow(clippy::manual_async_fn)]
        fn handle(
            _ctx: Context<Data>,
            _req: Request,
        ) -> impl Future<Output = Result<Response, ShioriError>> {
            async {
                let resp = Response::builder(v3::Version::SHIORI_30)
                    .status_code(v3::StatusCode::OK)
                    .build()?;
                Ok(resp.into())
            }
        }

        let handler = box_handler(handle);

        let ctx = Context::from(Data);
        let req = Request::builder(v3::Version::SHIORI_30)
            .method(v3::Method::GET)
            .header(v3::HeaderName::CHARSET, v3::Charset::UTF8.to_string())
            .build()?;
        let resp = handler.call(ctx, req.into()).await?;
        match resp {
            Response::V3(r) => {
                assert_eq!(r.version(), v3::Version::SHIORI_30);
                assert_eq!(r.status_code(), v3::StatusCode::OK);
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_box_handler_with_async_fn() -> Result<(), ShioriError> {
        async fn handle(_ctx: Context<Data>, _req: Request) -> Result<Response, ShioriError> {
            let resp = Response::builder(v3::Version::SHIORI_30)
                .status_code(v3::StatusCode::OK)
                .build()?;
            Ok(resp.into())
        }

        let handler = box_handler(handle);

        let ctx = Context::from(Data);
        let req = Request::builder(v3::Version::SHIORI_30)
            .method(v3::Method::GET)
            .header(v3::HeaderName::CHARSET, v3::Charset::UTF8.to_string())
            .build()?;
        let resp = handler.call(ctx, req.into()).await?;
        match resp {
            Response::V3(r) => {
                assert_eq!(r.version(), v3::Version::SHIORI_30);
                assert_eq!(r.status_code(), v3::StatusCode::OK);
            }
        }

        Ok(())
    }
}
