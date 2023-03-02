//! http://usada.sakura.vg/contents/sstp.html#notify11
//!
//! ```text
//! NOTIFY SSTP/1.1
//! Sender: さくら
//! Event: OnMusicPlay
//! Reference0: 元祖高木ブー伝説
//! Reference1: 筋肉少女帯
//! IfGhost: なる,ゆうか
//! Script: \h\s0‥‥\w8\w8高木ブーだね。\u\s0‥‥\e
//! IfGhost: さくら,うにゅう
//! Script: \h\s0‥‥\w8\w8高木ブーだね。\u\s0‥‥\w8\w8むう。\e
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
        .notify(Version::SSTP_11)
        .header(HeaderName::SENDER, "さくら")
        .header(HeaderName::EVENT, "OnMusicPlay")
        .header(HeaderName::REFERENCE0, "元祖高木ブー伝説")
        .header(HeaderName::REFERENCE1, "筋肉少女帯")
        .header(HeaderName::IF_GHOST, "なる,ゆうか")
        .header(
            HeaderName::SCRIPT,
            "\\h\\s0‥‥\\w8\\w8高木ブーだね。\\u\\s0‥‥\\e",
        )
        .header(HeaderName::IF_GHOST, "さくら,うにゅう")
        .header(
            HeaderName::SCRIPT,
            "\\h\\s0‥‥\\w8\\w8高木ブーだね。\\u\\s0‥‥\\w8\\w8むう。\\e",
        )
        .charset(Charset::SHIFT_JIS)
        .build()?;
    let input = request.as_bytes();

    let request = Request::parse(&input)?;
    assert_eq!(request.method(), Method::NOTIFY);
    assert_eq!(request.version(), Version::SSTP_11);
    assert_eq!(
        request
            .sender()
            .and_then(|v| v.text_with_charset(request.charset()).ok()),
        Some("さくら".to_string())
    );
    assert_eq!(
        request
            .event()
            .and_then(|v| v.text_with_charset(request.charset()).ok()),
        Some("OnMusicPlay".to_string())
    );
    assert_eq!(
        request
            .reference0()
            .and_then(|v| v.text_with_charset(request.charset()).ok()),
        Some("元祖高木ブー伝説".to_string())
    );
    assert_eq!(
        request
            .reference1()
            .and_then(|v| v.text_with_charset(request.charset()).ok()),
        Some("筋肉少女帯".to_string())
    );
    assert!(request.reference2().is_none());
    assert!(request.reference3().is_none());
    assert!(request.reference4().is_none());
    assert!(request.reference5().is_none());
    assert!(request.reference6().is_none());
    assert!(request.reference7().is_none());
    assert_eq!(
        request
            .if_ghost()
            .iter()
            .map(|v| v.text_with_charset(request.charset()).unwrap())
            .collect::<Vec<String>>(),
        vec!["なる,ゆうか", "さくら,うにゅう"]
    );
    assert_eq!(
        request
            .script()
            .iter()
            .map(|v| v.text_with_charset(request.charset()).unwrap())
            .collect::<Vec<String>>(),
        vec![
            "\\h\\s0‥‥\\w8\\w8高木ブーだね。\\u\\s0‥‥\\e",
            "\\h\\s0‥‥\\w8\\w8高木ブーだね。\\u\\s0‥‥\\w8\\w8むう。\\e"
        ]
    );
    assert_eq!(request.charset(), Charset::SHIFT_JIS);

    assert_eq!(request.as_bytes(), input);

    Ok(())
}
