//! http://usada.sakura.vg/contents/sstp.html#execute10
//!
//! ```text
//! EXECUTE SSTP/1.0
//! Sender: サンプルプログラム
//! Command: GetName
//! Charset: Shift_JIS
//!
//! [EOD]
//! ```
extern crate sstp_protocol;

use anyhow::Result;
use sstp_protocol::request::Request;
use sstp_protocol::{Charset, HeaderName, Method, Version};

fn main() -> Result<()> {
    let request = Request::builder()
        .execute(Version::SSTP_10)
        .header(HeaderName::SENDER, "サンプルプログラム")
        .header(HeaderName::COMMAND, "GetName")
        .charset(Charset::SHIFT_JIS)
        .build()?;
    let input = request.as_bytes();

    let request = Request::parse(&input)?;
    assert_eq!(request.method(), Method::EXECUTE);
    assert_eq!(request.version(), Version::SSTP_10);
    assert_eq!(
        request
            .sender()
            .and_then(|v| v.text_with_charset(request.charset()).ok()),
        Some("サンプルプログラム".to_string())
    );
    assert_eq!(
        request.command().and_then(|v| v.text().ok()),
        Some("GetName".to_string())
    );
    assert_eq!(request.charset(), Charset::SHIFT_JIS);

    assert_eq!(request.as_bytes(), input);

    Ok(())
}
