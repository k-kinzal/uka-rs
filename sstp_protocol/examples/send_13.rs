//! http://usada.sakura.vg/contents/sstp.html#send13
//!
//! ```text
//! SEND SSTP/1.3
//! Sender: カードキャプター
//! HWnd: 1024
//! Script: \h\s0どんな感じ？\n\n\q0[#temp0][まあまあ]\q1[#temp1][今ひとつ]\z
//! Entry: #temp0,\m[1025,0,0]\h\s0ふーん。\m[1025,0,1]\e
//! Entry: #temp1,\m[1025,1,0]\h\s0酒に逃げるなヨ！\m[1025,1,1]\e
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
        .send(Version::SSTP_13)
        .header(HeaderName::SENDER, "カードキャプター")
        .header(HeaderName::HWND, "1024")
        .header(
            HeaderName::SCRIPT,
            "\\h\\s0どんな感じ？\\n\\n\\q0[#temp0][まあまあ]\\q1[#temp1][今ひとつ]\\z",
        )
        .header(
            HeaderName::ENTRY,
            "#temp0,\\m[1025,0,0]\\h\\s0ふーん。\\m[1025,0,1]\\e",
        )
        .header(
            HeaderName::ENTRY,
            "#temp1,\\m[1025,1,0]\\h\\s0酒に逃げるなヨ！\\m[1025,1,1]\\e",
        )
        .charset(Charset::SHIFT_JIS)
        .build()?;
    let input = request.as_bytes();

    let request = Request::parse(&input)?;
    assert_eq!(request.method(), Method::SEND);
    assert_eq!(request.version(), Version::SSTP_13);
    assert_eq!(
        request
            .sender()
            .and_then(|v| v.text_with_charset(request.charset()).ok()),
        Some("カードキャプター".to_string())
    );
    assert_eq!(
        request.hwnd().and_then(|v| v.text().ok()),
        Some("1024".to_string())
    );
    assert_eq!(
        request
            .script()
            .iter()
            .map(|v| v.text_with_charset(request.charset()).unwrap())
            .collect::<Vec<String>>(),
        vec!["\\h\\s0どんな感じ？\\n\\n\\q0[#temp0][まあまあ]\\q1[#temp1][今ひとつ]\\z"]
    );
    assert_eq!(
        request
            .entry()
            .iter()
            .map(|v| v.text_with_charset(request.charset()).unwrap())
            .collect::<Vec<String>>(),
        vec![
            "#temp0,\\m[1025,0,0]\\h\\s0ふーん。\\m[1025,0,1]\\e",
            "#temp1,\\m[1025,1,0]\\h\\s0酒に逃げるなヨ！\\m[1025,1,1]\\e"
        ]
    );
    assert_eq!(request.charset(), Charset::SHIFT_JIS);

    assert_eq!(request.as_bytes(), input);

    Ok(())
}
