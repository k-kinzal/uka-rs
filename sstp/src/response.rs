use crate::header::{HeaderMap, HeaderName, HeaderNameError, HeaderValue, HeaderValueError};
use crate::parse::parse_response;
use crate::{Charset, StatusCode, Version};
use std::collections::HashMap;
use uka_util::decode::{Decoder, Error as DecodeError};
use uka_util::encode::{Encoder, Error as EncodeError};

/// AdditionalData is data that will be added in the SSTP response if needed.
///
/// This data is preserved in the character set and encoding specified by Charset.
/// To retrieve the data as a string, decode it according to the Charset.
#[derive(Debug)]
pub enum AdditionalData {
    Empty,
    Text(Vec<u8>),
}

impl AdditionalData {
    /// Extract HeaderValue as an ASCII code string.
    ///
    /// ```rust
    /// # use sstp::response::AdditionalData;
    /// let data = AdditionalData::from(b"sakura".to_vec());
    /// assert_eq!(data.text().unwrap(), "sakura");
    /// ```
    pub fn text(&self) -> Result<String, DecodeError> {
        self.text_with_charset(Charset::ASCII)
    }

    /// Extract HeaderValue as a string with Charset
    ///
    /// ```rust
    /// # use sstp::Charset;
    /// # use sstp::response::AdditionalData;
    /// let data = AdditionalData::from([130, 179, 130, 173, 130, 231].to_vec());
    /// assert_eq!(data.text_with_charset(Charset::SHIFT_JIS).unwrap(), "さくら");
    /// ```
    pub fn text_with_charset(&self, charset: Charset) -> Result<String, DecodeError> {
        match self {
            AdditionalData::Empty => Ok(String::new()),
            AdditionalData::Text(bytes) => {
                let v = match charset {
                    Charset::ASCII => Decoder::decode_ascii(bytes),
                    Charset::SHIFT_JIS => Decoder::decode_sjis(bytes),
                    Charset::ISO2022JP => Decoder::decode_iso_2022_jp(bytes),
                    Charset::EUC_JP => Decoder::decode_euc_jp(bytes),
                    Charset::UTF8 => Decoder::decode_utf8(bytes),
                };
                v.map(|s| s.trim().to_string())
            }
        }
    }

    /// Convert string to AdditionalData with ASCII code bytes.
    ///
    /// ```rust
    /// # use sstp::response::AdditionalData;
    /// let data = AdditionalData::from_static("sakura").unwrap();
    /// matches!(data, AdditionalData::Text(bytes) if bytes == b"sakura");
    /// ```
    pub fn from_static(s: &str) -> Result<Self, EncodeError> {
        Self::from_static_with_charset(s, Charset::ASCII)
    }

    ///　Convert string to AdditionalData with Charset.
    ///
    /// ```rust
    /// # use sstp::Charset;
    /// # use sstp::response::AdditionalData;
    /// let data = AdditionalData::from_static_with_charset("さくら", Charset::SHIFT_JIS).unwrap();
    /// matches!(data, AdditionalData::Text(bytes) if bytes == [130, 179, 130, 173, 130, 231]);
    /// ```
    pub fn from_static_with_charset(s: &str, charset: Charset) -> Result<Self, EncodeError> {
        if s.is_empty() {
            Ok(Self::Empty)
        } else {
            let s = s.replace('\n', "\r\n");
            let mut bytes = match charset {
                Charset::ASCII => Encoder::encode_ascii(s.as_str())?,
                Charset::SHIFT_JIS => Encoder::encode_sjis(s.as_str())?,
                Charset::ISO2022JP => Encoder::encode_iso_2022_jp(s.as_str())?,
                Charset::EUC_JP => Encoder::encode_euc_jp(s.as_str())?,
                Charset::UTF8 => Encoder::encode_utf8(s.as_str())?,
            };
            if &bytes[(bytes.len() - 1)..bytes.len()] != b"\r\n" {
                bytes.extend_from_slice(b"\r\n");
            }
            Ok(Self::Text(bytes))
        }
    }
}

impl From<&[u8]> for AdditionalData {
    fn from(s: &[u8]) -> Self {
        Self::from(s.to_vec())
    }
}

impl From<Vec<u8>> for AdditionalData {
    fn from(s: Vec<u8>) -> Self {
        if s.is_empty() {
            Self::Empty
        } else {
            Self::Text(s)
        }
    }
}

impl Default for AdditionalData {
    fn default() -> Self {
        Self::Empty
    }
}

/// Response is a type that represents an SSTP response.
///
/// Response provides a builder to generate types, a parser to generate types from bytes.
///
/// ```rust
/// # use sstp::{Charset, HeaderName, Method, StatusCode, Version};
/// # use sstp::response::Response;
/// let response = Response::builder()
///     .version(Version::SSTP_11)
///     .status_code(StatusCode::OK)
///     .charset(Charset::UTF8)
///     .header(
///         HeaderName::SCRIPT,
///         "\\h\\s0テストー。\\u\\s[10]テストやな。",
///     )
///     .additional("追加データはここ")
///     .build()
///     .unwrap();
/// assert_eq!(response.status_code(), StatusCode::OK);
/// ```
#[derive(Debug)]
pub struct Response {
    pub(crate) version: Version,
    pub(crate) status_code: StatusCode,
    pub(crate) headers: HeaderMap,
    pub(crate) charset: Charset,
    pub(crate) additional: AdditionalData,
}

impl Response {
    /// Parse a bytes into a Response.
    ///
    /// ```rust
    /// # use uka_util::encode::Encoder;
    /// # use sstp::{Charset, HeaderName, Method, StatusCode, Version};
    /// # use sstp::response::Response;
    /// let input = [
    ///     b"SSTP/1.1 200 OK\r\n".to_vec(),
    ///     b"Charset: UTF-8\r\n".to_vec(),
    ///     b"Script: ".to_vec(),
    ///     Encoder::encode_utf8("\\h\\s0テストー。\\u\\s[10]テストやな。").unwrap(),
    ///     b"\r\n".to_vec(),
    ///     b"\r\n".to_vec(),
    ///     Encoder::encode_utf8("追加データはここ").unwrap(),
    ///     b"\r\n".to_vec(),
    ///     b"\r\n".to_vec(),
    /// ].concat();
    /// let response = Response::parse(&input).unwrap();
    /// assert_eq!(response.status_code(), StatusCode::OK);
    /// ```
    pub fn parse(input: &[u8]) -> Result<Self, crate::Error> {
        parse_response(input)
    }

    /// Returns a builder that generates a type for the Response
    ///
    /// ```rust
    /// # use sstp::{Charset, HeaderName, Method, StatusCode, Version};
    /// # use sstp::response::Response;
    /// let response = Response::builder()
    ///     .version(Version::SSTP_11)
    ///     .status_code(StatusCode::OK)
    ///     .charset(Charset::UTF8)
    ///     .header(
    ///         HeaderName::SCRIPT,
    ///         "\\h\\s0テストー。\\u\\s[10]テストやな。",
    ///     )
    ///     .additional("追加データはここ")
    ///     .build()
    ///     .unwrap();
    /// assert_eq!(response.status_code(), StatusCode::OK);
    /// ```
    pub fn builder() -> Builder {
        Builder::new()
    }

    /// Returns SSTP version.
    pub fn version(&self) -> Version {
        self.version
    }

    /// Returns SSTP status code.
    pub fn status_code(&self) -> StatusCode {
        self.status_code
    }

    /// Returns SSTP header fields.
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    /// Returns SSTP charset.
    pub fn charset(&self) -> Charset {
        self.charset
    }

    /// Returns additional data.
    pub fn additional(&self) -> &AdditionalData {
        &self.additional
    }

    /// Convert request to bytes.
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
        if let AdditionalData::Text(bytes) = &self.additional {
            buf.extend_from_slice(bytes);
            buf.extend_from_slice(b"\r\n");
        }
        buf
    }
}

/// Error that can occur when build SSTP response.
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
    additional: Option<String>,
}

/// Builder for SSTP response.
pub struct Builder {
    inner: Result<Parts, Error>,
}

impl Builder {
    pub(crate) fn new() -> Self {
        Self {
            inner: Ok(Parts::default()),
        }
    }

    /// Set SSTP version.
    pub fn version(self, version: Version) -> Self {
        self.and_then(|parts| {
            Ok(Parts {
                version: Some(version),
                ..parts
            })
        })
    }

    /// Set SSTP status code.
    pub fn status_code(self, status_code: StatusCode) -> Self {
        self.and_then(|parts| {
            Ok(Parts {
                status_code: Some(status_code),
                ..parts
            })
        })
    }

    /// Set SSTP header field.
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

    /// Set SSTP charset.
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

    /// Set additional data.
    pub fn additional<T>(self, additional: T) -> Self
    where
        T: Into<String>,
    {
        self.and_then(|parts| {
            Ok(Parts {
                additional: Some(additional.into()),
                ..parts
            })
        })
    }

    /// Build SSTP response.
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
            additional: AdditionalData::from_static_with_charset(
                &inner.additional.unwrap_or_default(),
                charset,
            )?,
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
