use crate::types::v3::charset::Charset;
use crate::types::v3::header::{
    HeaderMap, HeaderName, HeaderNameError, HeaderValue, HeaderValueError,
};
use crate::types::v3::parse::{parse_response, Error as ParseError};
use crate::types::v3::status::StatusCode;
use crate::types::v3::version::Version;
use std::collections::HashMap;
use uka_util::encode::Error as EncodeError;

/// `Response` is a type that represents an SHIORI v3 request.
///
/// `Response` provides a builder to generate types, a parser to generate types from bytes,
/// and an accessor to the headers defined in the specification.
///
/// # Examples
///
/// ```rust
/// # use uka_shiori::types::v3::{Charset, HeaderName, Response, StatusCode, Version};
/// #
/// let response = Response::builder()
///   .version(Version::SHIORI_30)
///   .status_code(StatusCode::OK)
///   .header(HeaderName::SENDER, "F.I.R.S.T")
///   .header(HeaderName::VALUE, "hoge")
///   .charset(Charset::ASCII)
///   .build()
///   .unwrap();
/// assert_eq!(response.version(), Version::SHIORI_30);
/// ```
pub struct Response {
    pub(crate) version: Version,
    pub(crate) status_code: StatusCode,
    pub(crate) headers: HeaderMap,
    pub(crate) charset: Charset,
}

impl Response {
    /// Parse a bytes into a Response.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uka_shiori::types::v3::{Charset, HeaderName, Response, StatusCode, Version};
    /// #
    /// let input = [
    ///     b"SHIORI/3.0 204 No Content\r\n".to_vec(),
    ///     b"Sender: F.I.R.S.T\r\n".to_vec(),
    ///     b"Value: hoge\r\n".to_vec(),
    ///     b"Charset: UTF-8\r\n".to_vec(),
    ///     b"\r\n".to_vec(),
    /// ].concat();
    /// let response = Response::parse(&input).unwrap();
    /// assert_eq!(response.version(), Version::SHIORI_30);
    /// ```
    pub fn parse(buf: &[u8]) -> Result<Self, ParseError> {
        parse_response(buf)
    }

    pub fn builder() -> Builder {
        Builder::new()
    }

    /// Returns SHIORI version.
    pub fn version(&self) -> Version {
        self.version
    }

    /// Returns SHIORI status code.
    pub fn status_code(&self) -> StatusCode {
        self.status_code
    }

    /// Returns SHIORI header fields.
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    /// Returns SHIORI charset.
    pub fn charset(&self) -> Charset {
        self.charset
    }

    /// Sender
    pub fn sender(&self) -> Option<&HeaderValue> {
        self.headers.get(&HeaderName::SENDER)
    }

    /// ID
    pub fn id(&self) -> Option<&HeaderValue> {
        self.headers.get(&HeaderName::ID)
    }

    /// Reference0
    pub fn reference0(&self) -> Option<&HeaderValue> {
        self.headers.get(&HeaderName::REFERENCE0)
    }

    /// Reference1
    pub fn reference1(&self) -> Option<&HeaderValue> {
        self.headers.get(&HeaderName::REFERENCE1)
    }

    /// Reference2
    pub fn reference2(&self) -> Option<&HeaderValue> {
        self.headers.get(&HeaderName::REFERENCE2)
    }

    /// Reference3
    pub fn reference3(&self) -> Option<&HeaderValue> {
        self.headers.get(&HeaderName::REFERENCE3)
    }

    /// Reference4
    pub fn reference4(&self) -> Option<&HeaderValue> {
        self.headers.get(&HeaderName::REFERENCE4)
    }

    /// Reference5
    pub fn reference5(&self) -> Option<&HeaderValue> {
        self.headers.get(&HeaderName::REFERENCE5)
    }

    /// Reference6
    pub fn reference6(&self) -> Option<&HeaderValue> {
        self.headers.get(&HeaderName::REFERENCE6)
    }

    /// Reference7
    pub fn reference7(&self) -> Option<&HeaderValue> {
        self.headers.get(&HeaderName::REFERENCE7)
    }

    /// SecurityLevel
    pub fn security_level(&self) -> Option<&HeaderValue> {
        self.headers.get(&HeaderName::SECURITY_LEVEL)
    }

    /// Value
    pub fn value(&self) -> Option<&HeaderValue> {
        self.headers.get(&HeaderName::VALUE)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(self.version.to_string().as_bytes());
        buf.extend_from_slice(b" ");
        buf.extend_from_slice(self.status_code.to_string().as_bytes());
        buf.extend_from_slice(b"\r\n");
        for (name, value) in self.headers.iter() {
            buf.extend_from_slice(&name.as_bytes());
            buf.extend_from_slice(b": ");
            buf.extend_from_slice(&value.as_bytes());
            buf.extend_from_slice(b"\r\n");
        }
        buf.extend_from_slice(b"\r\n");
        buf
    }
}

/// Error that can occur when build SHIORI response.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("version is required")]
    MissingVersion,
    #[error("status_code is required")]
    MissingStatusCode,
    #[error("charset is required")]
    MissingCharset,
    #[error("{0}")]
    InvalidHeaderName(#[from] HeaderNameError),
    #[error("{0}")]
    FailedEncodeHeaderValue(#[from] HeaderValueError),
    #[error("{0}")]
    FailedEncodeAdditionalData(#[from] EncodeError),
}

#[derive(Default)]
struct Parts {
    version: Option<Version>,
    status_code: Option<StatusCode>,
    headers: HashMap<String, Vec<String>>,
    charset: Option<Charset>,
}

/// Builder for SHIORI response.
pub struct Builder {
    inner: Result<Parts, Error>,
}

impl Builder {
    pub(crate) fn new() -> Self {
        Self {
            inner: Ok(Parts::default()),
        }
    }

    /// Set SHIORI version.
    pub fn version(self, version: Version) -> Self {
        self.and_then(|parts| {
            Ok(Parts {
                version: Some(version),
                ..parts
            })
        })
    }

    /// Set SHIORI status code.
    pub fn status_code(self, status_code: StatusCode) -> Self {
        self.and_then(|parts| {
            Ok(Parts {
                status_code: Some(status_code),
                ..parts
            })
        })
    }

    /// Set SHIORI header field.
    pub fn header<K, V>(self, name: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.and_then(|mut inner| {
            inner
                .headers
                .entry(name.into())
                .or_default()
                .push(value.into());
            Ok(inner)
        })
    }

    /// Set SHIORI charset.
    pub fn charset(self, charset: Charset) -> Self {
        self.and_then(|mut inner| {
            inner
                .headers
                .entry(HeaderName::CHARSET.to_string())
                .or_default()
                .push(charset.to_string());
            if inner.charset.is_some() {
                Ok(Parts {
                    headers: inner.headers,
                    ..inner
                })
            } else {
                Ok(Parts {
                    charset: Some(charset),
                    headers: inner.headers,
                    ..inner
                })
            }
        })
    }

    /// Build SHIORI response.
    pub fn build(self) -> Result<Response, Error> {
        let inner = self.inner?;
        let charset = inner.charset.ok_or(Error::MissingCharset)?;
        Ok(Response {
            version: inner.version.ok_or(Error::MissingVersion)?,
            status_code: inner.status_code.ok_or(Error::MissingStatusCode)?,
            headers: inner.headers.iter().fold(
                Ok(HeaderMap::with_capacity(inner.headers.len())),
                |acc, (name, value)| {
                    value.iter().fold(acc, |acc, value| {
                        let name = HeaderName::from_static(name).map_err(Error::from);
                        let value = if value.chars().all(|c| c.is_ascii_graphic()) {
                            HeaderValue::from_static(value).map_err(Error::from)
                        } else {
                            HeaderValue::from_static_with_charset(value, charset)
                                .map_err(Error::from)
                        };
                        acc.and_then(|mut headers| {
                            name.and_then(|name| value.map(|value| (name, value))).map(
                                |(name, value)| {
                                    headers.insert(name, value);
                                    headers
                                },
                            )
                        })
                    })
                },
            )?,
            charset: inner.charset.ok_or(Error::MissingCharset)?,
        })
    }

    fn and_then<F>(self, func: F) -> Self
    where
        F: FnOnce(Parts) -> Result<Parts, Error>,
    {
        Builder {
            inner: self.inner.and_then(func),
        }
    }
}
