//! http://usada.sakura.vg/contents/sstp.html#send12
//!
//! ```text
//! SEND SSTP/1.2
//! Sender: カードキャプター
//! Script: \h\s0どんな感じ？\n\n\q0[#temp0][まあまあ]\q1[#temp1][今ひとつ]\z
//! Entry: #temp0,\h\s0ふーん。\e
//! Entry: #temp1,\h\s0酒に逃げるなヨ！\e
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
        .send(Version::SSTP_12)
        .header(HeaderName::SENDER, "カードキャプター")
        .header(
            HeaderName::SCRIPT,
            "\\h\\s0どんな感じ？\\n\\n\\q0[#temp0][まあまあ]\\q1[#temp1][今ひとつ]\\z",
        )
        .header(HeaderName::ENTRY, "#temp0,\\h\\s0ふーん。\\e")
        .header(HeaderName::ENTRY, "#temp1,\\h\\s0酒に逃げるなヨ！\\e")
        .charset(Charset::SHIFT_JIS)
        .build()?;
    let input = request.as_bytes();

    let request = Request::parse(&input)?;
    assert_eq!(request.method(), Method::SEND);
    assert_eq!(request.version(), Version::SSTP_12);
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
        vec!["\\h\\s0どんな感じ？\\n\\n\\q0[#temp0][まあまあ]\\q1[#temp1][今ひとつ]\\z"]
    );
    assert_eq!(
        request
            .entry()
            .iter()
            .map(|v| v.text_with_charset(request.charset()).unwrap())
            .collect::<Vec<String>>(),
        vec![
            "#temp0,\\h\\s0ふーん。\\e",
            "#temp1,\\h\\s0酒に逃げるなヨ！\\e"
        ]
    );
    assert_eq!(request.charset(), Charset::SHIFT_JIS);

    assert_eq!(request.as_bytes(), input);

    Ok(())
}
