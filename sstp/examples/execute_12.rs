//! http://usada.sakura.vg/contents/sstp.html#execute12
//!
//! ```text
//! EXECUTE SSTP/1.2
//! Sender: カードキャプター
//! Command: GetVersion
//! Charset: Shift_JIS
//!
//! [EOD]
//! ```
extern crate sstp;

use anyhow::Result;
use sstp::request::Request;
use sstp::{Charset, HeaderName, Method, Version};

fn main() -> Result<()> {
    let request = Request::builder()
        .execute(Version::SSTP_12)
        .header(HeaderName::SENDER, "カードキャプター")
        .header(HeaderName::COMMAND, "GetVersion")
        .charset(Charset::SHIFT_JIS)
        .build()?;
    let input = request.as_bytes();

    let request = Request::parse(&input)?;
    assert_eq!(request.method(), Method::EXECUTE);
    assert_eq!(request.version(), Version::SSTP_12);
    assert_eq!(
        request
            .sender()
            .and_then(|v| v.text_with_charset(request.charset()).ok()),
        Some("カードキャプター".to_string())
    );
    assert_eq!(
        request.command().and_then(|v| v.text().ok()),
        Some("GetVersion".to_string())
    );
    assert_eq!(request.charset(), Charset::SHIFT_JIS);

    assert_eq!(request.as_bytes(), input);

    Ok(())
}
