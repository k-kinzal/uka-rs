//! http://usada.sakura.vg/contents/sstp.html#execute11
//!
//! ```text
//! EXECUTE SSTP/1.1
//! Sender: カードキャプター
//! Command: SetCookie[visitcount,1]
//! Charset: Shift_JIS
//!
//! [EOD]
//!
//! EXECUTE SSTP/1.1
//! Sender: カードキャプター
//! Command: GetCookie[visitcount]
//! Charset: Shift_JIS
//!
//! [EOD]
//! ```
extern crate uka_sstp;

use anyhow::Result;
use uka_sstp::request::Request;
use uka_sstp::{Charset, HeaderName, Method, Version};

fn main() -> Result<()> {
    let request = Request::builder()
        .execute(Version::SSTP_11)
        .header(HeaderName::SENDER, "カードキャプター")
        .header(HeaderName::COMMAND, "SetCookie[visitcount,1]")
        .charset(Charset::SHIFT_JIS)
        .build()?;
    let input = request.as_bytes();

    let request = Request::parse(&input)?;
    assert_eq!(request.method(), Method::EXECUTE);
    assert_eq!(request.version(), Version::SSTP_11);
    assert_eq!(
        request
            .sender()
            .and_then(|v| v.text_with_charset(request.charset()).ok()),
        Some("カードキャプター".to_string())
    );
    assert_eq!(
        request.command().and_then(|v| v.text().ok()),
        Some("SetCookie[visitcount,1]".to_string())
    );
    assert_eq!(request.charset(), Charset::SHIFT_JIS);

    assert_eq!(request.as_bytes(), input);

    let request = Request::builder()
        .execute(Version::SSTP_11)
        .header(HeaderName::SENDER, "カードキャプター")
        .header(HeaderName::COMMAND, "GetCookie[visitcount]")
        .charset(Charset::SHIFT_JIS)
        .build()?;
    let input = request.as_bytes();

    let request = Request::parse(&input)?;
    assert_eq!(request.method(), Method::EXECUTE);
    assert_eq!(request.version(), Version::SSTP_11);
    assert_eq!(
        request
            .sender()
            .and_then(|v| v.text_with_charset(request.charset()).ok()),
        Some("カードキャプター".to_string())
    );
    assert_eq!(
        request.command().and_then(|v| v.text().ok()),
        Some("GetCookie[visitcount]".to_string())
    );
    assert_eq!(request.charset(), Charset::SHIFT_JIS);

    assert_eq!(request.as_bytes(), input);

    Ok(())
}
