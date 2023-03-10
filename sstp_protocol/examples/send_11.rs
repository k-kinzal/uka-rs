//! http://usada.sakura.vg/contents/sstp.html#send11
//!
//! ```text
//! SEND SSTP/1.1
//! Sender: カードキャプター
//! Script: \h\s0汝のあるべき姿に戻れ。\e
//! Option: nodescript,notranslate
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
        .send(Version::SSTP_11)
        .header(HeaderName::SENDER, "カードキャプター")
        .header(HeaderName::SCRIPT, "\\h\\s0汝のあるべき姿に戻れ。\\e")
        .header(HeaderName::OPTION, "nodescript,notranslate")
        .charset(Charset::SHIFT_JIS)
        .build()?;
    let input = request.as_bytes();

    let request = Request::parse(&input)?;
    assert_eq!(request.method(), Method::SEND);
    assert_eq!(request.version(), Version::SSTP_11);
    assert_eq!(
        request
            .sender()
            .and_then(|v| v.text_with_charset(request.charset()).ok()),
        Some("カードキャプター".to_string())
    );
    assert_eq!(
        request
            .script()
            .iter()
            .map(|v| v.text_with_charset(request.charset()).unwrap())
            .collect::<Vec<String>>(),
        vec!["\\h\\s0汝のあるべき姿に戻れ。\\e"]
    );
    assert_eq!(
        request.option().and_then(|v| v.text().ok()),
        Some("nodescript,notranslate".to_string())
    );
    assert_eq!(request.charset(), Charset::SHIFT_JIS);

    assert_eq!(request.as_bytes(), input);

    Ok(())
}
