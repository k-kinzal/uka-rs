use uka_shiori::types::v3::{Charset, HeaderName};
use uka_shiori::types::{Request, Response};
use uka_util::encode::Encoder;

/// This is an extended specification of uka-rs.
///
/// `Charset` is not referred to in the SHIORI/3.0 request.
/// However, to support multi-byte strings, the specification is inherited from SHIORI/2.x to support `Charset`.
/// The Charset value inherits the specification from SSTP and supports `ASCII`, `Shift_JIS`, `ISO-2022-JP`, `EUC-JP`, and `UTF-8`.
#[test]
fn spec_shiori_request_support_charset() -> anyhow::Result<()> {
    let input = [
        b"GET SHIORI/3.0\r\n".to_vec(),
        b"Charset: UTF-8\r\n".to_vec(),
        b"Sender: ".to_vec(),
        Encoder::encode_utf8("マテリア")?,
        b"\r\n".to_vec(),
        b"ID: hoge\r\n".to_vec(),
        b"Reference0: uge\r\n".to_vec(),
        b"\r\n".to_vec(),
    ]
    .concat();
    match Request::parse(&input).unwrap() {
        Request::V3(request) => {
            assert_eq!(request.charset(), Charset::UTF8);
            assert_eq!(
                request
                    .sender()
                    .and_then(|v| v.text_with_charset(Charset::UTF8).ok()),
                Some("マテリア".to_string())
            );
        }
    };

    Ok(())
}

/// This is an extended specification of uka-rs.
///
/// `Charset` is supported by the uka-rs extended specification, and `Charset` defaults to `ASCII`.
#[test]
fn spec_shiori_request_defaults_to_ascii_if_charset_is_omitted() -> anyhow::Result<()> {
    let input = [
        b"GET SHIORI/3.0\r\n".to_vec(),
        b"Sender: Materia\r\n".to_vec(),
        b"ID: hoge\r\n".to_vec(),
        b"Reference0: uge\r\n".to_vec(),
        b"\r\n".to_vec(),
    ]
    .concat();
    match Request::parse(&input)? {
        Request::V3(request) => {
            assert_eq!(request.charset(), Charset::ASCII);
        }
    };

    Ok(())
}

/// This is an extended specification of uka-rs.
///
/// The behavior when multiple headers are defined that are expected to be one is undefined in `Materia`.
/// In uka-rs, priority is given to the first header defined in accordance with the HTTP specification.
#[test]
fn spec_shiori_request_duplicate_single_headers_can_get_the_first_header() -> anyhow::Result<()> {
    let input = [
        b"GET SHIORI/3.0\r\n".to_vec(),
        b"Sender: Materia\r\n".to_vec(),
        b"Sender: Uka\r\n".to_vec(),
        b"ID: hoge\r\n".to_vec(),
        b"Reference0: uge\r\n".to_vec(),
        b"\r\n".to_vec(),
    ]
    .concat();
    match Request::parse(&input)? {
        Request::V3(request) => {
            assert_eq!(
                request
                    .sender()
                    .and_then(|v| v.text_with_charset(Charset::ASCII).ok()),
                Some("Materia".to_string())
            );
        }
    };
    Ok(())
}

/// This is an extended specification of uka-rs.
///
/// The behavior of the header key-value delimiter with no space is undefined in `Materia`.
/// uka-rs allows no spaces.
///
/// e.g. `HeaderName:HeaderValue`
#[test]
fn spec_shiori_request_allow_zero_spaces_after_the_header_delimiter() -> anyhow::Result<()> {
    let input = [
        b"GET SHIORI/3.0\r\n".to_vec(),
        b"Sender:Materia\r\n".to_vec(),
        b"ID:hoge\r\n".to_vec(),
        b"Reference0:uge\r\n".to_vec(),
        b"\r\n".to_vec(),
    ]
    .concat();
    match Request::parse(&input)? {
        Request::V3(request) => {
            assert_eq!(
                request
                    .sender()
                    .and_then(|v| v.text_with_charset(Charset::ASCII).ok()),
                Some("Materia".to_string())
            );
        }
    };
    Ok(())
}

/// This is an extended specification of uka-rs.
///
/// The behavior of the header key-value delimiter with more space is undefined in `Materia`.
/// uka-rs allows more spaces.
///
/// e.g. `HeaderName:  HeaderValue`
#[test]
fn spec_shiori_request_allow_one_or_more_spaces_after_the_header_delimiter() -> anyhow::Result<()> {
    let input = [
        b"GET SHIORI/3.0\r\n".to_vec(),
        b"Sender: Materia\r\n".to_vec(),
        b"ID:  hoge\r\n".to_vec(),
        b"Reference0:   uge\r\n".to_vec(),
        b"\r\n".to_vec(),
    ]
    .concat();
    match Request::parse(&input)? {
        Request::V3(request) => {
            assert_eq!(
                request
                    .sender()
                    .and_then(|v| v.text_with_charset(Charset::ASCII).ok()),
                Some("Materia".to_string())
            );
        }
    };
    Ok(())
}

/// This is an extended specification of uka-rs.
///
/// In Materia, the characters allowed in the header are undefined.
/// only certain printable characters are allowed according to the HTTP specification.
#[test]
fn spec_shiori_request_character_set_for_request_header_names_is_based_on_rfc_7230(
) -> anyhow::Result<()> {
    let name = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!#$%&'*+-.^_`|~";

    let input = [
        b"GET SHIORI/3.0\r\n".to_vec(),
        format!("{name}: Materia\r\n").as_bytes().to_vec(),
        b"ID: hoge\r\n".to_vec(),
        b"Reference0: uge\r\n".to_vec(),
        b"\r\n".to_vec(),
    ]
    .concat();

    match Request::parse(&input)? {
        Request::V3(request) => {
            assert_eq!(
                request
                    .headers()
                    .get(&HeaderName::from_static(name)?)
                    .and_then(|v| v.text_with_charset(Charset::ASCII).ok()),
                Some("Materia".to_string())
            );
        }
    };
    Ok(())
}

/// This is an extended specification of uka-rs.
///
/// `Charset` is not referred to in the SHIORI/3.0 response.
/// However, to support multi-byte strings, the specification is inherited from SHIORI/2.x to support `Charset`.
/// The Charset value inherits the specification from SSTP and supports `ASCII`, `Shift_JIS`, `ISO-2022-JP`, `EUC-JP`, and `UTF-8`.
#[test]
fn spec_shiori_response_support_charset() -> anyhow::Result<()> {
    let input = [
        b"SHIORI/3.0 200 OK\r\n".to_vec(),
        b"Charset: UTF-8\r\n".to_vec(),
        b"Sender: ".to_vec(),
        Encoder::encode_utf8("マテリア")?,
        b"\r\n".to_vec(),
        b"Value: hoge\r\n".to_vec(),
        b"\r\n".to_vec(),
    ]
    .concat();
    match Response::parse(&input).unwrap() {
        Response::V3(response) => {
            assert_eq!(response.charset(), Charset::UTF8);
            assert_eq!(
                response
                    .sender()
                    .and_then(|v| v.text_with_charset(Charset::UTF8).ok()),
                Some("マテリア".to_string())
            );
        }
    };

    Ok(())
}

/// This is an extended specification of uka-rs.
///
/// `Charset` is supported by the uka-rs extended specification, and `Charset` defaults to `ASCII`.
#[test]
fn spec_shiori_response_defaults_to_ascii_if_charset_is_omitted() -> anyhow::Result<()> {
    let input = [
        b"SHIORI/3.0 200 OK\r\n".to_vec(),
        b"Sender: F.I.R.S.T\r\n".to_vec(),
        b"Value: hoge\r\n".to_vec(),
        b"\r\n".to_vec(),
    ]
    .concat();
    match Response::parse(&input)? {
        Response::V3(response) => {
            assert_eq!(response.charset(), Charset::ASCII);
        }
    };

    Ok(())
}

/// This is an extended specification of uka-rs.
///
/// The behavior when multiple headers are defined that are expected to be one is undefined in `Materia`.
/// In uka-rs, priority is given to the first header defined in accordance with the HTTP specification.
#[test]
fn spec_shiori_response_duplicate_single_headers_can_get_the_first_header() -> anyhow::Result<()> {
    let input = [
        b"SHIORI/3.0 200 OK\r\n".to_vec(),
        b"Sender: F.I.R.S.T\r\n".to_vec(),
        b"Sender: S.E.C.O.N.D\r\n".to_vec(),
        b"Value: hoge\r\n".to_vec(),
        b"\r\n".to_vec(),
    ]
    .concat();
    match Response::parse(&input)? {
        Response::V3(response) => {
            assert_eq!(
                response
                    .sender()
                    .and_then(|v| v.text_with_charset(Charset::ASCII).ok()),
                Some("F.I.R.S.T".to_string())
            );
        }
    };
    Ok(())
}

/// This is an extended specification of uka-rs.
///
/// The behavior of the header key-value delimiter with no space is undefined in `Materia`.
/// uka-rs allows no spaces.
///
/// e.g. `HeaderName:HeaderValue`
#[test]
fn spec_shiori_response_allow_zero_spaces_after_the_header_delimiter() -> anyhow::Result<()> {
    let input = [
        b"SHIORI/3.0 200 OK\r\n".to_vec(),
        b"Sender:F.I.R.S.T\r\n".to_vec(),
        b"Value:hoge\r\n".to_vec(),
        b"\r\n".to_vec(),
    ]
    .concat();
    match Response::parse(&input)? {
        Response::V3(response) => {
            assert_eq!(
                response
                    .sender()
                    .and_then(|v| v.text_with_charset(Charset::ASCII).ok()),
                Some("F.I.R.S.T".to_string())
            );
        }
    };
    Ok(())
}

/// This is an extended specification of uka-rs.
///
/// The behavior of the header key-value delimiter with more space is undefined in `Materia`.
/// uka-rs allows more spaces.
///
/// e.g. `HeaderName:  HeaderValue`
#[test]
fn spec_shiori_response_allow_one_or_more_spaces_after_the_header_delimiter() -> anyhow::Result<()>
{
    let input = [
        b"SHIORI/3.0 200 OK\r\n".to_vec(),
        b"Sender: F.I.R.S.T\r\n".to_vec(),
        b"Value:   hoge\r\n".to_vec(),
        b"\r\n".to_vec(),
    ]
    .concat();
    match Response::parse(&input)? {
        Response::V3(response) => {
            assert_eq!(
                response
                    .value()
                    .and_then(|v| v.text_with_charset(Charset::ASCII).ok()),
                Some("hoge".to_string())
            );
        }
    };
    Ok(())
}

/// This is an extended specification of uka-rs.
///
/// In Materia, the characters allowed in the header are undefined.
/// only certain printable characters are allowed according to the HTTP specification.
#[test]
fn spec_shiori_response_character_set_for_response_header_names_is_based_on_rfc_7230(
) -> anyhow::Result<()> {
    let name = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!#$%&'*+-.^_`|~";

    let input = [
        b"SHIORI/3.0 200 OK\r\n".to_vec(),
        format!("{name}: Materia\r\n").as_bytes().to_vec(),
        b"Value:   hoge\r\n".to_vec(),
        b"\r\n".to_vec(),
    ]
    .concat();

    match Response::parse(&input)? {
        Response::V3(response) => {
            assert_eq!(
                response
                    .headers()
                    .get(&HeaderName::from_static(name)?)
                    .and_then(|v| v.text_with_charset(Charset::ASCII).ok()),
                Some("Materia".to_string())
            );
        }
    };
    Ok(())
}
