//! http://usada.sakura.vg/contents/sstp.html#communicate12
//!
//! ```text
//! COMMUNICATE SSTP/1.2
//! Sender: 双葉
//! HWnd: 0
//! Sentence: \0\s0どうも。\e
//! Surface: 0,10
//! Reference0: N/A
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
        .communicate(Version::SSTP_12)
        .header(HeaderName::SENDER, "双葉")
        .header(HeaderName::HWND, "0")
        .header(HeaderName::SENTENCE, "\\0\\s0どうも。\\e")
        .header(HeaderName::SURFACE, "0,10")
        .header(HeaderName::REFERENCE0, "N/A")
        .charset(Charset::SHIFT_JIS)
        .build()?;
    let input = request.as_bytes();

    let request = Request::parse(&input)?;
    assert_eq!(request.method(), Method::COMMUNICATE);
    assert_eq!(request.version(), Version::SSTP_12);
    assert_eq!(
        request
            .sender()
            .and_then(|v| v.text_with_charset(request.charset()).ok()),
        Some("双葉".to_string())
    );
    assert_eq!(
        request.hwnd().and_then(|v| v.text().ok()),
        Some("0".to_string())
    );
    assert_eq!(
        request
            .sentence()
            .and_then(|v| v.text_with_charset(request.charset()).ok()),
        Some("\\0\\s0どうも。\\e".to_string())
    );
    assert_eq!(
        request.surface().and_then(|v| v.text().ok()),
        Some("0,10".to_string())
    );
    assert_eq!(
        request.reference0().and_then(|v| v.text().ok()),
        Some("N/A".to_string())
    );
    assert_eq!(request.charset(), Charset::SHIFT_JIS);

    assert_eq!(request.as_bytes(), input);

    Ok(())
}
