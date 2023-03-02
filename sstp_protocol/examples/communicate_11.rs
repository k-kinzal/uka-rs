//! http://usada.sakura.vg/contents/sstp.html#communicate11
//!
//! ```text
//! COMMUNICATE SSTP/1.1
//! Sender: カードキャプター
//! Sentence: 今日は寒いなー。
//! Option: substitute
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
        .communicate(Version::SSTP_11)
        .header(HeaderName::SENDER, "カードキャプター")
        .header(HeaderName::SENTENCE, "今日は寒いなー。")
        .header(HeaderName::OPTION, "substitute")
        .charset(Charset::SHIFT_JIS)
        .build()?;
    let input = request.as_bytes();

    let request = Request::parse(&input)?;
    assert_eq!(request.method(), Method::COMMUNICATE);
    assert_eq!(request.version(), Version::SSTP_11);
    assert_eq!(
        request
            .sender()
            .and_then(|v| v.text_with_charset(request.charset()).ok()),
        Some("カードキャプター".to_string())
    );
    assert_eq!(
        request
            .sentence()
            .and_then(|v| v.text_with_charset(request.charset()).ok()),
        Some("今日は寒いなー。".to_string())
    );
    assert_eq!(
        request
            .option()
            .and_then(|v| v.text_with_charset(request.charset()).ok()),
        Some("substitute".to_string())
    );
    assert_eq!(request.charset(), Charset::SHIFT_JIS);

    assert_eq!(request.as_bytes(), input);

    Ok(())
}
