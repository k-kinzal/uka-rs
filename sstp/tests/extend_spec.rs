extern crate sstp;

use anyhow::Result;
use encoding_rs::SHIFT_JIS;
use sstp::request::Request;
use sstp::response::Response;
use sstp::{Charset, HeaderName, StatusCode, Version};

/// Undefined specification for materia.
///
/// If multiple headers are defined that are expected to be a single header,
/// the value of the first occurrence of the header is obtained according to the general HTTP server specification.
#[test]
fn spec_duplicate_single_headers_can_get_the_first_header() -> Result<()> {
    let request = Request::builder()
        .send(Version::SSTP_11)
        .header(HeaderName::SENDER, "カードキャプター")
        .header(HeaderName::SCRIPT, "\\h\\s0汝のあるべき姿に戻れ。\\e")
        .header(HeaderName::OPTION, "nodescript,notranslate")
        .charset(Charset::UTF8)
        .charset(Charset::SHIFT_JIS)
        .build()?;
    let input = request.as_bytes();

    assert_eq!(
        String::from_utf8(input.clone())?
            .match_indices("Charset")
            .count(),
        2
    );

    let request = Request::parse(&input)?;
    assert_eq!(request.charset(), Charset::UTF8);

    Ok(())
}

/// Undefined specification for materia.
///
/// Zero spaces after the header delimiter are allowed.
/// e.g. `HeaderName:HeaderValue`
#[test]
fn spec_allow_zero_spaces_after_the_header_delimiter() -> Result<()> {
    let input = [
        b"SEND SSTP/1.1\r\n".to_vec(),
        b"Sender:".to_vec(),
        SHIFT_JIS.encode("カードキャプター").0.to_vec(),
        b"\r\n".to_vec(),
        b"Script:".to_vec(),
        SHIFT_JIS
            .encode("\\h\\s0汝のあるべき姿に戻れ。\\e")
            .0
            .to_vec(),
        b"\r\n".to_vec(),
        b"Option:nodescript,notranslate\r\n".to_vec(),
        b"Charset:Shift_JIS\r\n".to_vec(),
        b"\r\n".to_vec(),
    ]
    .concat();

    let request = Request::parse(&input)?;
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

    Ok(())
}

/// Undefined specification for materia.
///
/// More than one space after the header delimiter is allowed.
/// e.g. `HeaderName:  HeaderValue`
/// Whitespace after the delimiter is trimmed.
#[test]
fn spec_allow_one_or_more_spaces_after_the_header_delimiter() -> Result<()> {
    let input = [
        b"SEND SSTP/1.1\r\n".to_vec(),
        b"Sender: ".to_vec(),
        SHIFT_JIS.encode("カードキャプター").0.to_vec(),
        b"\r\n".to_vec(),
        b"Script:  ".to_vec(),
        SHIFT_JIS
            .encode("\\h\\s0汝のあるべき姿に戻れ。\\e")
            .0
            .to_vec(),
        b"\r\n".to_vec(),
        b"Option:   nodescript,notranslate\r\n".to_vec(),
        b"Charset:    Shift_JIS\r\n".to_vec(),
        b"\r\n".to_vec(),
    ]
    .concat();

    let request = Request::parse(&input)?;
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

    Ok(())
}

/// Undefined specification for materia.
///
/// In materia, headers are defined as consisting only of ASCII codes.
/// However, since control characters are not acceptable,
/// only certain printable characters are allowed according to the HTTP specification.
#[test]
fn spec_character_set_for_request_header_names_is_based_on_rfc_7230() -> Result<()> {
    let name = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!#$%&'*+-.^_`|~";

    let request = Request::builder()
        .send(Version::SSTP_11)
        .header(HeaderName::from_static(name)?, "foo")
        .charset(Charset::SHIFT_JIS)
        .build()?;
    let input = request.as_bytes();

    let request = Request::parse(&input)?;
    assert_eq!(
        request
            .headers()
            .get(HeaderName::from_static(name)?)
            .unwrap()
            .text()?,
        "foo"
    );

    Ok(())
}

/// Undefined specification for materia.
///
/// In materia, headers are defined as consisting only of ASCII codes.
/// however, since control characters are not acceptable,
/// only certain printable characters are allowed according to the HTTP specification.
#[test]
fn spec_character_set_for_response_header_names_is_based_on_rfc_7230() -> Result<()> {
    let name = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!#$%&'*+-.^_`|~";

    let request = Request::builder()
        .send(Version::SSTP_11)
        .header(HeaderName::from_static(name)?, "foo")
        .charset(Charset::SHIFT_JIS)
        .build()?;
    let input = request.as_bytes();

    let request = Request::parse(&input)?;
    assert_eq!(
        request
            .headers()
            .get(HeaderName::from_static(name)?)
            .unwrap()
            .text()?,
        "foo"
    );

    Ok(())
}

/// Undefined specification for materia.
///
/// The materiel defines the header value to be ASCII or the character code or character encoding specified in Charset.
/// However, since control characters are not acceptable,
/// A string containing control characters cannot be specified in the header value to avoid problems with parsing.
#[test]
fn spec_request_header_value_cannot_contain_control_characters() -> Result<()> {
    let request = Request::builder()
        .send(Version::SSTP_11)
        .header(HeaderName::SENDER, "foo\0bar")
        .charset(Charset::SHIFT_JIS)
        .build();
    assert!(request.is_err());

    Ok(())
}

/// Undefined specification for materia.
///
/// The materiel defines the header value to be ASCII or the character code or character encoding specified in Charset.
/// However, since control characters are not acceptable,
/// A string containing control characters cannot be specified in the header value to avoid problems with parsing.
#[test]
fn spec_response_header_value_cannot_contain_control_characters() -> Result<()> {
    let response = Response::builder()
        .status_code(StatusCode::OK)
        .header(HeaderName::SENDER, "foo\0bar")
        .charset(Charset::SHIFT_JIS)
        .build();
    assert!(response.is_err());

    Ok(())
}
