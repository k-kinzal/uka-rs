//! http://usada.sakura.vg/contents/sstp.html#give11
//!
//! ```text
//! GIVE SSTP/1.1
//! Sender: カードキャプター
//! Document: こんにちはさくらです。闇の力を秘めし鍵よ真の姿を我の前に示せレリーズ。汝のあるべき姿に戻れクロウカード。
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
        .give(Version::SSTP_11)
        .header(HeaderName::SENDER, "カードキャプター")
        .header(HeaderName::DOCUMENT, "こんにちはさくらです。闇の力を秘めし鍵よ真の姿を我の前に示せレリーズ。汝のあるべき姿に戻れクロウカード。")
        .charset(Charset::SHIFT_JIS)
        .build()?;
    let input = request.as_bytes();

    let request = Request::parse(&input)?;
    assert_eq!(request.method(), Method::GIVE);
    assert_eq!(request.version(), Version::SSTP_11);
    assert_eq!(
        request
            .sender()
            .and_then(|v| v.text_with_charset(request.charset()).ok()),
        Some("カードキャプター".to_string())
    );
    assert_eq!(
        request
            .document()
            .and_then(|v| v.text_with_charset(request.charset()).ok()),
        Some("こんにちはさくらです。闇の力を秘めし鍵よ真の姿を我の前に示せレリーズ。汝のあるべき姿に戻れクロウカード。".to_string()));
    assert_eq!(request.charset(), Charset::SHIFT_JIS);

    assert_eq!(request.as_bytes(), input);

    Ok(())
}
