use crate::context::ShioriContext;
use std::future::Future;
use std::pin::Pin;
use uka_shiori::runtime::{Context, Service};
use uka_shiori::types::v3;

async fn get_on_anchor_select(
    ctx: Context<ShioriContext>,
    event: v3::OnAnchorSelect,
) -> Result<String, v3::ShioriError> {
    Ok("\\0あーあ\\w1押しちゃった\\e".to_string())
}

async fn notify_on_anchor_select(
    _ctx: Context<ShioriContext>,
    _event: v3::OnAnchorSelect,
) -> Result<(), v3::ShioriError> {
    Ok(())
}

#[derive(Default)]
pub struct Handler;
impl Service<ShioriContext, v3::Request> for Handler {
    type Response = v3::Response;
    type Error = v3::ShioriError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn call(&self, context: Context<ShioriContext>, request: v3::Request) -> Self::Future {
        let fut = async {
            let charset = request.charset();
            match request.method() {
                v3::Method::GET => {
                    let value = match v3::Event::try_from(request)? {
                        v3::Event::OnAnchorSelect(ev) => get_on_anchor_select(context, ev).await?,
                        v3::Event::UndefinedEvent(ev) => unimplemented!(),
                    };
                    Ok(v3::Response::builder()
                        .version(v3::Version::SHIORI_30)
                        .status_code(v3::StatusCode::OK)
                        .charset(charset)
                        .header(v3::HeaderName::SENDER, "Sakura")
                        .header(v3::HeaderName::VALUE, value)
                        .build()?)
                }
                v3::Method::NOTIFY => {
                    match v3::Event::try_from(request)? {
                        v3::Event::OnAnchorSelect(ev) => {
                            notify_on_anchor_select(context, ev).await?
                        }
                        v3::Event::UndefinedEvent(ev) => unimplemented!(),
                    };
                    Ok(v3::Response::builder()
                        .version(v3::Version::SHIORI_30)
                        .status_code(v3::StatusCode::NO_CONTENT)
                        .build()?)
                }
            }
        };
        Box::pin(fut)
    }
}
