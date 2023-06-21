//! http://usada.sakura.vg/contents/sstp.html#send14
//!
//! ```text
//! SEND SSTP/1.4
//! Sender: カードキャプター
//! IfGhost: さくら,うにゅう
//! Script: \h\s0さくらだー。\w8\n\n%j[#mainblock]
//! IfGhost: せりこ,まるちい
//! Script: \h\s0せりこだー。\w8\n\n%j[#mainblock]
//! IfGhost: さくら,ケロ
//! Script: \u\s0わいのはモダン焼きにしてや～。\w8\h\s0はいはい。\e
//! Entry: #mainblock,\s7寝言は寝てから言えっ！\w8\u\s0落ち着けっ！\e
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
        .send(Version::SSTP_14)
        .header(HeaderName::SENDER, "カードキャプター")
        .header(HeaderName::IF_GHOST, "さくら,うにゅう")
        .header(
            HeaderName::SCRIPT,
            "\\h\\s0さくらだー。\\w8\\n\\n%j[#mainblock]",
        )
        .header(HeaderName::IF_GHOST, "せりこ,まるちい")
        .header(
            HeaderName::SCRIPT,
            "\\h\\s0せりこだー。\\w8\\n\\n%j[#mainblock]",
        )
        .header(HeaderName::IF_GHOST, "さくら,ケロ")
        .header(
            HeaderName::SCRIPT,
            "\\u\\s0わいのはモダン焼きにしてや～。\\w8\\h\\s0はいはい。\\e",
        )
        .header(
            HeaderName::ENTRY,
            "#mainblock,\\s7寝言は寝てから言えっ！\\w8\\u\\s0落ち着けっ！\\e",
        )
        .charset(Charset::SHIFT_JIS)
        .build()?;
    let input = request.as_bytes();

    let request = Request::parse(&input)?;
    assert_eq!(request.method(), Method::SEND);
    assert_eq!(request.version(), Version::SSTP_14);
    assert_eq!(
        request
            .sender()
            .and_then(|v| v.text_with_charset(request.charset()).ok()),
        Some("カードキャプター".to_string())
    );
    assert_eq!(
        request
            .if_ghost()
            .iter()
            .map(|v| v.text_with_charset(request.charset()).unwrap())
            .collect::<Vec<String>>(),
        vec!["さくら,うにゅう", "せりこ,まるちい", "さくら,ケロ"]
    );
    assert_eq!(
        request
            .script()
            .iter()
            .map(|v| v.text_with_charset(request.charset()).unwrap())
            .collect::<Vec<String>>(),
        vec![
            "\\h\\s0さくらだー。\\w8\\n\\n%j[#mainblock]",
            "\\h\\s0せりこだー。\\w8\\n\\n%j[#mainblock]",
            "\\u\\s0わいのはモダン焼きにしてや～。\\w8\\h\\s0はいはい。\\e"
        ]
    );
    assert_eq!(
        request
            .entry()
            .iter()
            .map(|v| v.text_with_charset(request.charset()).unwrap())
            .collect::<Vec<String>>(),
        vec!["#mainblock,\\s7寝言は寝てから言えっ！\\w8\\u\\s0落ち着けっ！\\e"]
    );
    assert_eq!(request.charset(), Charset::SHIFT_JIS);

    assert_eq!(request.as_bytes(), input);

    Ok(())
}
