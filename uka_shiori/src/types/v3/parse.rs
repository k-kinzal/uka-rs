use crate::types::v3::charset::{Charset, Error as CharsetError};
use crate::types::v3::header::{HeaderMap, HeaderName, HeaderNameError, HeaderValueError};
use crate::types::v3::method::Method;
use crate::types::v3::request::Request;
use crate::types::v3::response::Response;
use crate::types::v3::status::StatusCode;
use crate::types::v3::version::Version;
use crate::types::v3::ParseError;
use std::io::Cursor;
use uka_util::cursor::{lookahead, read_expect, read_match, read_repeat, read_until};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    IO(#[from] std::io::Error),

    #[error("invalid header name: {0:?}")]
    InvalidHeaderName(#[from] HeaderNameError),

    #[error("`{0}` header not found")]
    MissingHeader(HeaderName),

    #[error("{1} in `{0}` header")]
    FailedDecode(HeaderName, #[source] HeaderValueError),

    #[error("{0}")]
    UnsupportedCharset(#[from] CharsetError),

    #[error("unexpected eof")]
    UnexpectedEof,
}

type Result<T> = std::result::Result<T, Error>;

pub fn parse_request(input: &[u8]) -> Result<Request> {
    let mut cursor = Cursor::new(input);
    let method = parse_method(&mut cursor)?;
    skip_spaces(&mut cursor)?;
    let version = parse_version(&mut cursor)?;
    skip_newline(&mut cursor)?;
    let headers = parse_headers(&mut cursor)?;
    let charset = headers
        .get(&HeaderName::CHARSET)
        .ok_or(Error::MissingHeader(HeaderName::CHARSET))
        .and_then(|v| {
            v.text()
                .map_err(|e| Error::FailedDecode(HeaderName::CHARSET, e))
        })
        .and_then(|v| Charset::from_string(v).map_err(Error::from))
        .or(Ok::<Charset, ParseError>(Charset::ASCII))?;
    skip_newline(&mut cursor)?;
    eof(&mut cursor)?;

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
    skip_spaces(&mut cursor)?;
    let status_code = parse_status_code(&mut cursor)?;
    skip_newline(&mut cursor)?;
    let headers = parse_headers(&mut cursor)?;
    let charset = headers
        .get(&HeaderName::CHARSET)
        .ok_or(Error::MissingHeader(HeaderName::CHARSET))
        .and_then(|v| {
            v.text()
                .map_err(|e| Error::FailedDecode(HeaderName::CHARSET, e))
        })
        .and_then(|v| Charset::from_string(v).map_err(Error::from))
        .or(Ok::<Charset, ParseError>(Charset::ASCII))?;
    skip_newline(&mut cursor)?;
    eof(&mut cursor)?;

    Ok(Response {
        version,
        status_code,
        headers,
        charset,
    })
}

fn parse_method(cursor: &mut Cursor<&[u8]>) -> Result<Method> {
    read_match!(cursor, {
        b"GET" => Method::GET,
        b"NOTIFY" => Method::NOTIFY,
    })
    .map_err(Error::from)
}

fn parse_version(cursor: &mut Cursor<&[u8]>) -> Result<Version> {
    read_match!(cursor, {
        b"SHIORI/3.0" => Version::SHIORI_30,
    })
    .map_err(Error::from)
}

fn parse_status_code(cursor: &mut Cursor<&[u8]>) -> Result<StatusCode> {
    read_match!(cursor, {
        b"200 OK" => StatusCode::OK,
        b"204 No Content" => StatusCode::NO_CONTENT,
        b"310 Communicate" => StatusCode::COMMUNICATE,
        b"311 Not Enough" => StatusCode::NOT_ENOUGH,
        b"312 Advice" => StatusCode::ADVICE,
        b"400 Bad Request" => StatusCode::BAD_REQUEST,
        b"500 Internal Server Error" => StatusCode::INTERNAL_SERVER_ERROR,
    }, 3)
    .map_err(Error::from)
}

fn parse_headers(cursor: &mut Cursor<&[u8]>) -> Result<HeaderMap> {
    let mut map = HeaderMap::new();
    loop {
        let buffer = lookahead!(cursor, 2).map_err(Error::from)?;
        if &buffer == b"\r\n" {
            break;
        }
        let name = read_until!(cursor, b":").map_err(Error::from)?;
        skip_spaces(cursor)?;
        let value = read_until!(cursor, b"\r\n").map_err(Error::from)?;

        map.insert(
            HeaderName::from_bytes(&name).map_err(Error::from)?,
            value.into(),
        )
    }
    Ok(map)
}

fn skip_spaces(cursor: &mut Cursor<&[u8]>) -> Result<()> {
    read_repeat!(cursor, b" ").map_err(Error::from)?;
    Ok(())
}

fn skip_newline(cursor: &mut Cursor<&[u8]>) -> Result<()> {
    read_expect!(cursor, b"\r\n").map_err(Error::from)?;
    Ok(())
}

fn eof(cursor: &mut Cursor<&[u8]>) -> Result<()> {
    if cursor.position() as usize == cursor.get_ref().len() {
        Ok(())
    } else {
        Err(Error::UnexpectedEof)
    }
}
