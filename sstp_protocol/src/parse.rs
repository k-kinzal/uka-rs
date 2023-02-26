use crate::header::{HeaderMap, HeaderName};
use crate::method::Method;
use crate::request::Request;
use crate::response::{AdditionalData, Response};
use crate::version::Version;
use crate::{charset, decode, header, Charset, StatusCode};
use std::io;
use std::io::{Cursor, Read, Seek};
use std::num::ParseIntError;
use std::str::Utf8Error;

/// Error that can occur when parse from bytes.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("read error: {0:?}")]
    Io(#[from] io::Error),

    #[error("invalid method: please use `NOTIFY`or `SEND`, `EXECUTE`, `GIVE`, `COMMUNICATE` for the method")]
    InvalidMethod,

    #[error("invalid version: please use `SSTP/1.0`or `SSTP/1.1`, `SSTP/1.2`, `SSTP/1.3`, `SSTP/1.4` for the version")]
    InvalidVersion,

    #[error(
        "invalid status code: please use 2xx, 4xx, or 5xx status codes based on the specification"
    )]
    InvalidStatusCode,

    #[error("invalid header name: {0:?}")]
    InvalidHeaderName(#[from] header::Error),

    #[error("invalid header: {0:?}: {1:?}")]
    InvalidHeaderValue(HeaderName, String),

    #[error("`{0}` header not found")]
    MissingHeader(HeaderName),

    #[error("{1} in `{0}` header")]
    FailedDecode(HeaderName, #[source] decode::Error),

    #[error("{0}")]
    UnsupportedCharset(#[from] charset::Error),

    #[error("{0}")]
    FailedUtf8Decode(#[from] Utf8Error),

    #[error("{0}")]
    FailedParseInt(#[from] ParseIntError),
}

pub type Result<T> = std::result::Result<T, Error>;

macro_rules! read {
    ( $cursor:expr, $byte:expr ) => {{
        let mut buffer = [0; $byte];
        $cursor.read_exact(&mut buffer[..]).map(|_| buffer)
    }};
}

macro_rules! read_until {
    ( $cursor:expr, $byte:expr ) => {{
        let mut buffer = Vec::new();
        loop {
            let mut buf = [0; $byte.len()];
            if let Err(e) = $cursor.read_exact(&mut buf) {
                break Err(e);
            } else if &buf == $byte {
                break Ok(buffer);
            } else {
                if $byte.len() > 1 {
                    if let Err(e) =
                        $cursor.seek(std::io::SeekFrom::Current(-($byte.len() as i64 - 1)))
                    {
                        break Err(e);
                    }
                }
                buffer.push(buf[0]);
            }
        }
    }};
}

macro_rules! read_repeat {
    ( $cursor:expr, $byte:expr ) => {{
        let mut buffer = Vec::new();
        loop {
            let mut buf = [0; $byte.len()];
            if let Err(e) = $cursor.read_exact(&mut buf) {
                break Err(e);
            } else if &buf != $byte {
                if let Err(e) = $cursor.seek(std::io::SeekFrom::Current(-1)) {
                    break Err(e);
                } else {
                    break Ok(buffer);
                }
            } else {
                buffer.append(&mut buf.to_vec());
            }
        }
    }};
}

macro_rules! lookahead {
    ( $cursor:expr, $byte:expr ) => {{
        let mut buffer = [0; $byte];
        $cursor
            .read_exact(&mut buffer[..])
            .and_then(|_| $cursor.seek(std::io::SeekFrom::Current(-$byte as i64)))
            .map(|_| buffer)
    }};
}

pub fn parse_request(input: &[u8]) -> Result<Request> {
    let mut cursor = Cursor::new(input);

    let method = parse_method(&mut cursor)?;
    let version = parse_version(&mut cursor)?;
    let headers = parse_headers(&mut cursor)?;
    let charset = headers
        .get(HeaderName::CHARSET)
        .ok_or_else(|| Error::MissingHeader(HeaderName::CHARSET))
        .and_then(|v| {
            v.text()
                .map_err(|e| Error::FailedDecode(HeaderName::CHARSET, e))
        })
        .and_then(|v| Charset::from_string(v).map_err(Error::from))?;

    Ok(Request {
        method,
        version,
        headers,
        charset,
    })
}

pub fn parse_response(input: &[u8]) -> Result<Response> {
    let mut cursor = Cursor::new(input);

    let version = parse_version(&mut cursor)?;
    let status_code = parse_status_code(&mut cursor)?;
    let headers = parse_headers(&mut cursor)?;
    let additional = parse_additional_data(&mut cursor)?;
    let charset = headers
        .get(HeaderName::CHARSET)
        .ok_or_else(|| Error::MissingHeader(HeaderName::CHARSET))
        .and_then(|v| {
            v.text()
                .map_err(|e| Error::FailedDecode(HeaderName::CHARSET, e))
        })
        .and_then(|v| Charset::from_string(v).map_err(Error::from))?;

    Ok(Response {
        version,
        status_code,
        headers,
        additional,
        charset,
    })
}

fn parse_method(cursor: &mut Cursor<&[u8]>) -> Result<Method> {
    let char = read!(cursor, 1).map_err(Error::from)?;
    match &char {
        b"N" => read!(cursor, 6)
            .map_err(Error::from)
            .and_then(|buf| match &buf {
                b"OTIFY " => Ok(Method::NOTIFY),
                _ => Err(Error::InvalidMethod),
            }),
        b"S" => read!(cursor, 4)
            .map_err(Error::from)
            .and_then(|buf| match &buf {
                b"END " => Ok(Method::SEND),
                _ => Err(Error::InvalidMethod),
            }),
        b"E" => read!(cursor, 7)
            .map_err(Error::from)
            .and_then(|buf| match &buf {
                b"XECUTE " => Ok(Method::EXECUTE),
                _ => Err(Error::InvalidMethod),
            }),
        b"G" => read!(cursor, 4)
            .map_err(Error::from)
            .and_then(|buf| match &buf {
                b"IVE " => Ok(Method::GIVE),
                _ => Err(Error::InvalidMethod),
            }),
        b"C" => read!(cursor, 11)
            .map_err(Error::from)
            .and_then(|buf| match &buf {
                b"OMMUNICATE " => Ok(Method::COMMUNICATE),
                _ => Err(Error::InvalidMethod),
            }),
        _ => Err(Error::InvalidMethod),
    }
}

fn parse_version(cursor: &mut Cursor<&[u8]>) -> Result<Version> {
    let buffer = read!(cursor, 8).map_err(Error::from)?;
    match &buffer {
        b"SSTP/1.0" => {
            let buffer = read!(cursor, 1).map_err(Error::from)?;
            match &buffer {
                b" " => Ok(Version::SSTP_10),
                b"\r" => {
                    let buffer = read!(cursor, 1).map_err(Error::from)?;
                    if &buffer == b"\n" {
                        Ok(Version::SSTP_10)
                    } else {
                        Err(Error::InvalidVersion)
                    }
                }
                _ => Err(Error::InvalidVersion),
            }
        }
        b"SSTP/1.1" => {
            let buffer = read!(cursor, 1).map_err(Error::from)?;
            match &buffer {
                b" " => Ok(Version::SSTP_11),
                b"\r" => {
                    let buffer = read!(cursor, 1).map_err(Error::from)?;
                    if &buffer == b"\n" {
                        Ok(Version::SSTP_11)
                    } else {
                        Err(Error::InvalidVersion)
                    }
                }
                _ => Err(Error::InvalidVersion),
            }
        }
        b"SSTP/1.2" => {
            let buffer = read!(cursor, 1).map_err(Error::from)?;
            match &buffer {
                b" " => Ok(Version::SSTP_12),
                b"\r" => {
                    let buffer = read!(cursor, 1).map_err(Error::from)?;
                    if &buffer == b"\n" {
                        Ok(Version::SSTP_12)
                    } else {
                        Err(Error::InvalidVersion)
                    }
                }
                _ => Err(Error::InvalidVersion),
            }
        }
        b"SSTP/1.3" => {
            let buffer = read!(cursor, 1).map_err(Error::from)?;
            match &buffer {
                b" " => Ok(Version::SSTP_13),
                b"\r" => {
                    let buffer = read!(cursor, 1).map_err(Error::from)?;
                    if &buffer == b"\n" {
                        Ok(Version::SSTP_13)
                    } else {
                        Err(Error::InvalidVersion)
                    }
                }
                _ => Err(Error::InvalidVersion),
            }
        }
        b"SSTP/1.4" => {
            let buffer = read!(cursor, 1).map_err(Error::from)?;
            match &buffer {
                b" " => Ok(Version::SSTP_14),
                b"\r" => {
                    let buffer = read!(cursor, 1).map_err(Error::from)?;
                    if &buffer == b"\n" {
                        Ok(Version::SSTP_14)
                    } else {
                        Err(Error::InvalidVersion)
                    }
                }
                _ => Err(Error::InvalidVersion),
            }
        }
        _ => Err(Error::InvalidVersion),
    }
}

fn parse_status_code(cursor: &mut Cursor<&[u8]>) -> Result<StatusCode> {
    let buffer = read!(cursor, 4).map_err(Error::from)?;
    match &buffer {
        b"200 " => {
            let buffer = read!(cursor, 4).map_err(Error::from)?;
            if &buffer == b"OK\r\n" {
                Ok(StatusCode::OK)
            } else {
                Err(Error::InvalidStatusCode)
            }
        }
        b"204 " => {
            let buffer = read!(cursor, 12).map_err(Error::from)?;
            if &buffer == b"No Content\r\n" {
                Ok(StatusCode::NO_CONTENT)
            } else {
                Err(Error::InvalidStatusCode)
            }
        }
        b"210 " => {
            let buffer = read!(cursor, 7).map_err(Error::from)?;
            if &buffer == b"Break\r\n" {
                Ok(StatusCode::BREAK)
            } else {
                Err(Error::InvalidStatusCode)
            }
        }
        b"400 " => {
            let buffer = read!(cursor, 13).map_err(Error::from)?;
            if &buffer == b"Bad Request\r\n" {
                Ok(StatusCode::BAD_REQUEST)
            } else {
                Err(Error::InvalidStatusCode)
            }
        }
        b"408 " => {
            let buffer = read!(cursor, 17).map_err(Error::from)?;
            if &buffer == b"Request Timeout\r\n" {
                Ok(StatusCode::REQUEST_TIMEOUT)
            } else {
                Err(Error::InvalidStatusCode)
            }
        }
        b"409 " => {
            let buffer = read!(cursor, 10).map_err(Error::from)?;
            if &buffer == b"Conflict\r\n" {
                Ok(StatusCode::CONFLICT)
            } else {
                Err(Error::InvalidStatusCode)
            }
        }
        b"420 " => {
            let buffer = read!(cursor, 8).map_err(Error::from)?;
            if &buffer == b"Refuse\r\n" {
                Ok(StatusCode::REFUSE)
            } else {
                Err(Error::InvalidStatusCode)
            }
        }
        b"501 " => {
            let buffer = read!(cursor, 17).map_err(Error::from)?;
            if &buffer == b"Not Implemented\r\n" {
                Ok(StatusCode::NOT_IMPLEMENTED)
            } else {
                Err(Error::InvalidStatusCode)
            }
        }
        b"503 " => {
            let buffer = read!(cursor, 21).map_err(Error::from)?;
            if &buffer == b"Service Unavailable\r\n" {
                Ok(StatusCode::SERVICE_UNAVAILABLE)
            } else {
                Err(Error::InvalidStatusCode)
            }
        }
        b"510 " => {
            let buffer = read!(cursor, 14).map_err(Error::from)?;
            if &buffer == b"Not Local IP\r\n" {
                Ok(StatusCode::NOT_LOCAL_IP)
            } else {
                Err(Error::InvalidStatusCode)
            }
        }
        b"511 " => {
            let buffer = read!(cursor, 15).map_err(Error::from)?;
            if &buffer == b"In Black List\r\n" {
                Ok(StatusCode::IN_BLACK_LIST)
            } else {
                Err(Error::InvalidStatusCode)
            }
        }
        b"512 " => {
            let buffer = read!(cursor, 11).map_err(Error::from)?;
            if &buffer == b"Invisible\r\n" {
                Ok(StatusCode::INVISIBLE)
            } else {
                Err(Error::InvalidStatusCode)
            }
        }
        _ => Err(Error::InvalidStatusCode),
    }
}

fn parse_headers(cursor: &mut Cursor<&[u8]>) -> Result<HeaderMap> {
    let mut map = HeaderMap::new();
    loop {
        let buffer = lookahead!(cursor, 2).map_err(Error::from)?;
        if &buffer == b"\r\n" {
            let _ = read!(cursor, 2);
            break;
        }

        let name = read_until!(cursor, b":").map_err(Error::from)?;
        let name = HeaderName::from_bytes(name.as_slice()).map_err(Error::from)?;

        let _ = read_repeat!(cursor, b" ").map_err(Error::from)?;

        let value = read_until!(cursor, b"\r\n").map_err(Error::from)?;

        map.insert(name, value)
    }

    Ok(map)
}

fn parse_additional_data(cursor: &mut Cursor<&[u8]>) -> Result<AdditionalData> {
    let result = lookahead!(cursor, 1);
    if result.is_err() && result.unwrap_err().kind() == std::io::ErrorKind::UnexpectedEof {
        return Ok(AdditionalData::Empty);
    }
    let mut bytes = Vec::new();
    loop {
        let buffer = lookahead!(cursor, 2).map_err(Error::from)?;
        if &buffer == b"\r\n" {
            let _ = read!(cursor, 2);
            break;
        }
        let buffer = read_until!(cursor, b"\r\n").map_err(Error::from)?;
        bytes.extend(buffer);
        bytes.extend(b"\r\n");
    }

    Ok(AdditionalData::Text(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encode::Encoder;
    use anyhow::Result;

    #[test]
    fn test_parse_method_pass_notify() -> Result<()> {
        let mut cursor = Cursor::new(b"NOTIFY ".as_slice());
        let method = parse_method(&mut cursor)?;

        assert_eq!(method, Method::NOTIFY);

        Ok(())
    }

    #[test]
    fn test_parse_method_pass_send() -> Result<()> {
        let mut cursor = Cursor::new(b"SEND ".as_slice());
        let method = parse_method(&mut cursor)?;

        assert_eq!(method, Method::SEND);

        Ok(())
    }

    #[test]
    fn test_parse_method_pass_execute() -> Result<()> {
        let mut cursor = Cursor::new(b"EXECUTE ".as_slice());
        let method = parse_method(&mut cursor)?;

        assert_eq!(method, Method::EXECUTE);

        Ok(())
    }

    #[test]
    fn test_parse_method_pass_give() -> Result<()> {
        let mut cursor = Cursor::new(b"GIVE ".as_slice());
        let method = parse_method(&mut cursor)?;

        assert_eq!(method, Method::GIVE);

        Ok(())
    }

    #[test]
    fn test_parse_method_pass_communicate() -> Result<()> {
        let mut cursor = Cursor::new(b"COMMUNICATE ".as_slice());
        let method = parse_method(&mut cursor)?;

        assert_eq!(method, Method::COMMUNICATE);

        Ok(())
    }

    #[test]
    fn test_parse_method_pass_undefined() -> Result<()> {
        let mut cursor = Cursor::new(b"FOO ".as_slice());
        let res = parse_method(&mut cursor);

        assert!(res.is_err());
        matches!(res.unwrap_err(), Error::InvalidMethod);

        Ok(())
    }

    #[test]
    fn test_parse_version_pass_sstp_1_0() -> Result<()> {
        let mut cursor = Cursor::new(b"SSTP/1.0\r\n".as_slice());
        let version = parse_version(&mut cursor)?;

        assert_eq!(version, Version::SSTP_10);

        Ok(())
    }

    #[test]
    fn test_parse_version_pass_sstp_1_1() -> Result<()> {
        let mut cursor = Cursor::new(b"SSTP/1.1\r\n".as_slice());
        let version = parse_version(&mut cursor)?;

        assert_eq!(version, Version::SSTP_11);

        Ok(())
    }

    #[test]
    fn test_parse_version_pass_sstp_1_2() -> Result<()> {
        let mut cursor = Cursor::new(b"SSTP/1.2\r\n".as_slice());
        let version = parse_version(&mut cursor)?;

        assert_eq!(version, Version::SSTP_12);

        Ok(())
    }

    #[test]
    fn test_parse_version_pass_sstp_1_3() -> Result<()> {
        let mut cursor = Cursor::new(b"SSTP/1.3\r\n".as_slice());
        let version = parse_version(&mut cursor)?;

        assert_eq!(version, Version::SSTP_13);

        Ok(())
    }

    #[test]
    fn test_parse_version_pass_sstp_1_4() -> Result<()> {
        let mut cursor = Cursor::new(b"SSTP/1.4\r\n".as_slice());
        let version = parse_version(&mut cursor)?;

        assert_eq!(version, Version::SSTP_14);

        Ok(())
    }

    #[test]
    fn test_parse_version_pass_undefined_version() -> Result<()> {
        let mut cursor = Cursor::new(b"SSTP/0.1\r\n".as_slice());
        let res = parse_version(&mut cursor);

        assert!(res.is_err());
        matches!(res.unwrap_err(), Error::InvalidVersion);

        Ok(())
    }

    #[test]
    fn test_parse_status_code_200() -> Result<()> {
        let mut cursor = Cursor::new(b"200 OK\r\n".as_slice());
        let code = parse_status_code(&mut cursor)?;
        assert_eq!(code, StatusCode::OK);
        Ok(())
    }

    #[test]
    fn test_parse_status_code_204() -> Result<()> {
        let mut cursor = Cursor::new(b"204 No Content\r\n".as_slice());
        let code = parse_status_code(&mut cursor)?;
        assert_eq!(code, StatusCode::NO_CONTENT);
        Ok(())
    }

    #[test]
    fn test_parse_status_code_210() -> Result<()> {
        let mut cursor = Cursor::new(b"210 Break\r\n".as_slice());
        let code = parse_status_code(&mut cursor)?;
        assert_eq!(code, StatusCode::BREAK);
        Ok(())
    }

    #[test]
    fn test_parse_status_code_400() -> Result<()> {
        let mut cursor = Cursor::new(b"400 Bad Request\r\n".as_slice());
        let code = parse_status_code(&mut cursor)?;
        assert_eq!(code, StatusCode::BAD_REQUEST);
        Ok(())
    }

    #[test]
    fn test_parse_status_code_408() -> Result<()> {
        let mut cursor = Cursor::new(b"408 Request Timeout\r\n".as_slice());
        let code = parse_status_code(&mut cursor)?;
        assert_eq!(code, StatusCode::REQUEST_TIMEOUT);
        Ok(())
    }

    #[test]
    fn test_parse_status_code_409() -> Result<()> {
        let mut cursor = Cursor::new(b"409 Conflict\r\n".as_slice());
        let code = parse_status_code(&mut cursor)?;
        assert_eq!(code, StatusCode::CONFLICT);
        Ok(())
    }

    #[test]
    fn test_parse_status_code_420() -> Result<()> {
        let mut cursor = Cursor::new(b"420 Refuse\r\n".as_slice());
        let code = parse_status_code(&mut cursor)?;
        assert_eq!(code, StatusCode::REFUSE);
        Ok(())
    }

    #[test]
    fn test_parse_status_code_501() -> Result<()> {
        let mut cursor = Cursor::new(b"501 Not Implemented\r\n".as_slice());
        let code = parse_status_code(&mut cursor)?;
        assert_eq!(code, StatusCode::NOT_IMPLEMENTED);
        Ok(())
    }

    #[test]
    fn test_parse_status_code_503() -> Result<()> {
        let mut cursor = Cursor::new(b"503 Service Unavailable\r\n".as_slice());
        let code = parse_status_code(&mut cursor)?;
        assert_eq!(code, StatusCode::SERVICE_UNAVAILABLE);
        Ok(())
    }

    #[test]
    fn test_parse_status_code_510() -> Result<()> {
        let mut cursor = Cursor::new(b"510 Not Local IP\r\n".as_slice());
        let code = parse_status_code(&mut cursor)?;
        assert_eq!(code, StatusCode::NOT_LOCAL_IP);
        Ok(())
    }

    #[test]
    fn test_parse_status_code_511() -> Result<()> {
        let mut cursor = Cursor::new(b"511 In Black List\r\n".as_slice());
        let code = parse_status_code(&mut cursor)?;
        assert_eq!(code, StatusCode::IN_BLACK_LIST);
        Ok(())
    }

    #[test]
    fn test_parse_status_code_512() -> Result<()> {
        let mut cursor = Cursor::new(b"512 Invisible\r\n".as_slice());
        let code = parse_status_code(&mut cursor)?;
        assert_eq!(code, StatusCode::INVISIBLE);
        Ok(())
    }

    #[test]
    fn test_parse_status_code_undefined() -> Result<()> {
        let mut cursor = Cursor::new(b"999 Undefine Code\r\n".as_slice());
        let res = parse_status_code(&mut cursor);
        assert!(res.is_err());
        matches!(res.unwrap_err(), Error::InvalidStatusCode);

        Ok(())
    }

    #[test]
    fn test_parse_header_name_with_alpha() -> Result<()> {
        let input = [
            b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ: foo\r\n".to_vec(),
            b"\r\n".to_vec(),
        ]
        .concat();
        let mut cursor = Cursor::new(input.as_slice());
        let headers = parse_headers(&mut cursor)?;

        let value = headers
            .get(HeaderName::from_static(
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ",
            )?)
            .and_then(|v| v.text().ok())
            .unwrap();
        assert_eq!(value, "foo");

        Ok(())
    }

    #[test]
    fn test_parse_header_name_with_digit() -> Result<()> {
        let input = [b"0123456789: foo\r\n".to_vec(), b"\r\n".to_vec()].concat();
        let mut cursor = Cursor::new(input.as_slice());
        let headers = parse_headers(&mut cursor)?;

        let value = headers
            .get(HeaderName::from_static("0123456789")?)
            .and_then(|v| v.text().ok())
            .unwrap();
        assert_eq!(value, "foo");

        Ok(())
    }

    #[test]
    fn test_parse_header_name_with_symbol() -> Result<()> {
        let input = [b"!#$%&'*+-.^_`|~: foo\r\n".to_vec(), b"\r\n".to_vec()].concat();
        let mut cursor = Cursor::new(input.as_slice());
        let headers = parse_headers(&mut cursor)?;

        let value = headers
            .get(HeaderName::from_static("!#$%&'*+-.^_`|~")?)
            .and_then(|v| v.text().ok())
            .unwrap();
        assert_eq!(value, "foo");

        Ok(())
    }

    #[test]
    fn test_parse_header_name_with_invalid_symbol() -> Result<()> {
        let input = [b"(),/;<=>?@[\\]{}: foo\r\n".to_vec(), b"\r\n".to_vec()].concat();
        let mut cursor = Cursor::new(input.as_slice());
        let res = parse_headers(&mut cursor);

        assert!(res.is_err());
        matches!(res.unwrap_err(), Error::InvalidHeaderName(_));

        Ok(())
    }

    #[test]
    fn test_parse_header_ascii_value() -> Result<()> {
        let input = [
            b"Sender: ".to_vec(),
            Encoder::encode_ascii("foo")?.to_vec(),
            b"\r\n".to_vec(),
            b"\r\n".to_vec(),
        ]
        .concat();
        let mut cursor = Cursor::new(input.as_slice());
        let headers = parse_headers(&mut cursor)?;

        let sender = headers
            .get(HeaderName::SENDER)
            .and_then(|v| v.text().ok())
            .unwrap();
        assert_eq!(sender, "foo");

        Ok(())
    }

    #[test]
    fn test_parse_header_sjis_value() -> Result<()> {
        let input = [
            b"Sender: ".to_vec(),
            Encoder::encode_sjis("あいうえお")?.to_vec(),
            b"\r\n".to_vec(),
            b"\r\n".to_vec(),
        ]
        .concat();
        let mut cursor = Cursor::new(input.as_slice());
        let headers = parse_headers(&mut cursor)?;

        let sender = headers
            .get(HeaderName::SENDER)
            .and_then(|v| v.text_with_charset(Charset::SHIFT_JIS).ok())
            .unwrap();
        assert_eq!(sender, "あいうえお");

        Ok(())
    }

    #[test]
    fn test_parse_header_iso_2022_jp_value() -> Result<()> {
        let input = [
            b"Sender: ".to_vec(),
            Encoder::encode_iso_2022_jp("あいうえお")?.to_vec(),
            b"\r\n".to_vec(),
            b"\r\n".to_vec(),
        ]
        .concat();
        let mut cursor = Cursor::new(input.as_slice());
        let headers = parse_headers(&mut cursor)?;

        let sender = headers
            .get(HeaderName::SENDER)
            .and_then(|v| v.text_with_charset(Charset::ISO2022JP).ok())
            .unwrap();
        assert_eq!(sender, "あいうえお");

        Ok(())
    }

    #[test]
    fn test_parse_header_euc_jp_value() -> Result<()> {
        let input = [
            b"Sender: ".to_vec(),
            Encoder::encode_euc_jp("あいうえお")?.to_vec(),
            b"\r\n".to_vec(),
            b"\r\n".to_vec(),
        ]
        .concat();
        let mut cursor = Cursor::new(input.as_slice());
        let headers = parse_headers(&mut cursor)?;

        let sender = headers
            .get(HeaderName::SENDER)
            .and_then(|v| v.text_with_charset(Charset::EUC_JP).ok())
            .unwrap();
        assert_eq!(sender, "あいうえお");

        Ok(())
    }

    #[test]
    fn test_parse_header_utf8_value() -> Result<()> {
        let input = [
            b"Sender: ".to_vec(),
            Encoder::encode_utf8("あいうえお")?.to_vec(),
            b"\r\n".to_vec(),
            b"\r\n".to_vec(),
        ]
        .concat();
        let mut cursor = Cursor::new(input.as_slice());
        let headers = parse_headers(&mut cursor)?;

        let sender = headers
            .get(HeaderName::SENDER)
            .and_then(|v| v.text_with_charset(Charset::UTF8).ok())
            .unwrap();
        assert_eq!(sender, "あいうえお");

        Ok(())
    }

    #[test]
    fn test_parse_header_multiline() -> Result<()> {
        let input = [
            b"Sender: foo\r\n".to_vec(),
            b"Charset: ASCII\r\n".to_vec(),
            b"\r\n".to_vec(),
        ]
        .concat();
        let mut cursor = Cursor::new(input.as_slice());
        let headers = parse_headers(&mut cursor)?;

        let sender = headers
            .get(HeaderName::SENDER)
            .and_then(|v| v.text().ok())
            .unwrap();
        let charset = headers
            .get(HeaderName::CHARSET)
            .and_then(|v| v.text().ok())
            .unwrap();
        assert_eq!(sender, "foo");
        assert_eq!(charset, "ASCII");

        Ok(())
    }

    #[test]
    fn test_parse_header_no_last_newline() -> Result<()> {
        let input = [b"Sender: foo\r\n".to_vec(), b"Charset: ASCII\r\n".to_vec()].concat();
        let mut cursor = Cursor::new(input.as_slice());
        let res = parse_headers(&mut cursor);

        assert!(res.is_err());
        matches!(res.unwrap_err(), Error::Io(_));

        Ok(())
    }

    #[test]
    fn test_parse_additional_data_empty() -> Result<()> {
        let input = b"";
        let mut cursor = Cursor::new(input.as_slice());
        let additional = parse_additional_data(&mut cursor)?;

        matches!(additional, AdditionalData::Empty);

        Ok(())
    }

    #[test]
    fn test_parse_additional_data_one_line() -> Result<()> {
        let input = b"line1\r\n\r\n";
        let mut cursor = Cursor::new(input.as_slice());
        let additional = parse_additional_data(&mut cursor)?;

        matches!(additional, AdditionalData::Text(bytes) if bytes == b"line1");

        Ok(())
    }

    #[test]
    fn test_parse_additional_data_multi_line() -> Result<()> {
        let input = b"line1\r\nline2\r\n\r\n";
        let mut cursor = Cursor::new(input.as_slice());
        let additional = parse_additional_data(&mut cursor)?;

        matches!(additional, AdditionalData::Text(bytes) if bytes == b"line1\r\nline2");

        Ok(())
    }

    #[test]
    fn test_parse_request() -> Result<()> {
        let input = [
            b"NOTIFY SSTP/1.1\r\n".to_vec(),
            b"Sender: foo\r\n".to_vec(),
            b"Charset: ASCII\r\n".to_vec(),
            b"\r\n".to_vec(),
        ]
        .concat();
        let request = parse_request(&input)?;

        assert_eq!(request.method(), Method::NOTIFY);
        assert_eq!(request.version(), Version::SSTP_11);
        assert_eq!(request.sender().and_then(|v| v.text().ok()).unwrap(), "foo");
        assert_eq!(request.charset(), Charset::ASCII);

        Ok(())
    }

    #[test]
    fn test_parse_request_missing_charset() -> Result<()> {
        let input = [
            b"NOTIFY SSTP/1.1\r\n".to_vec(),
            b"Sender: foo\r\n".to_vec(),
            b"\r\n".to_vec(),
        ]
        .concat();
        let res = parse_request(&input);

        assert!(res.is_err());
        matches!(res.unwrap_err(), Error::MissingHeader(HeaderName::CHARSET));

        Ok(())
    }

    #[test]
    fn test_parse_request_not_ascii_charset() -> Result<()> {
        let input = [
            b"NOTIFY SSTP/1.1\r\n".to_vec(),
            b"Sender: foo\r\n".to_vec(),
            b"Charset: ".to_vec(),
            Encoder::encode_utf8("あいうえお")?.to_vec(),
            b"\r\n".to_vec(),
            b"\r\n".to_vec(),
        ]
        .concat();
        let res = parse_request(&input);

        assert!(res.is_err());
        matches!(
            res.unwrap_err(),
            Error::FailedDecode(HeaderName::CHARSET, _)
        );

        Ok(())
    }

    #[test]
    fn test_parse_request_unsupported_charset() -> Result<()> {
        let input = [
            b"NOTIFY SSTP/1.1\r\n".to_vec(),
            b"Sender: foo\r\n".to_vec(),
            b"Charset: ISO-8859-16\r\n".to_vec(),
            b"\r\n".to_vec(),
        ]
        .concat();
        let res = parse_request(&input);

        assert!(res.is_err());
        matches!(res.unwrap_err(), Error::UnsupportedCharset(_));

        Ok(())
    }

    #[test]
    fn test_parse_request_no_last_newline() -> Result<()> {
        let input = [
            b"NOTIFY SSTP/1.1\r\n".to_vec(),
            b"Sender: foo\r\n".to_vec(),
            b"Charset: ASCII\r\n".to_vec(),
        ]
        .concat();
        let res = parse_request(&input);

        assert!(res.is_err());
        matches!(res.unwrap_err(), Error::Io(_));

        Ok(())
    }
}
