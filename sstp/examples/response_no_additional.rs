//! https://ssp.shillest.net/ukadoc/manual/spec_sstp.html#req_res
//! ```test
//!
//! SSTP/1.4 200 OK
//! Charset: UTF-8
//! Script: \h\s0テストー。\u\s[10]テストやな。
//!
//! [EOD]
//! ```
extern crate sstp;

use anyhow::Result;
use sstp::response::{AdditionalData, Response};
use sstp::{Charset, HeaderName, StatusCode, Version};

fn main() -> Result<()> {
    let response = Response::builder()
        .version(Version::SSTP_14)
        .status_code(StatusCode::OK)
        .charset(Charset::UTF8)
        .header(
            HeaderName::SCRIPT,
            "\\h\\s0テストー。\\u\\s[10]テストやな。",
        )
        .build()?;
    let input = response.as_bytes();

    let response = Response::parse(&input)?;
    assert_eq!(response.version(), Version::SSTP_14);
    assert_eq!(response.status_code(), StatusCode::OK);
    assert_eq!(response.charset(), Charset::UTF8);
    assert_eq!(
        response
            .headers()
            .get(&HeaderName::SCRIPT)
            .and_then(|v| v.text_with_charset(response.charset()).ok()),
        Some("\\h\\s0テストー。\\u\\s[10]テストやな。".to_string())
    );
    matches!(response.additional(), AdditionalData::Empty);
    assert_eq!(response.additional().text()?, "");

    assert_eq!(response.as_bytes(), input);

    Ok(())
}
