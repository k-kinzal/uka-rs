use crate::types::v3::charset::Charset;
use crate::types::v3::header::{
    HeaderMap, HeaderName, HeaderNameError, HeaderValue, HeaderValueError,
};
use crate::types::v3::parse::{parse_request, Error as ParseError};
use crate::types::v3::{Method, Version};
use uka_util::bag::OrderedBag;

/// Request is a type that represents an SHIORI v3 request.
///
/// Request provides a builder to generate types, a parser to generate types from bytes,
/// and an accessor to the headers defined in the specification.
///
/// # Examples
///
/// ```rust
/// # use uka_shiori::types::v3::{Charset, HeaderName, Method, Version, Request};
/// let request = Request::builder()
///     .method(Method::GET)
///     .version(Version::SHIORI_30)
///     .header(HeaderName::SENDER, "Materia")
///     .header(HeaderName::ID, "hoge")
///     .header(HeaderName::REFERENCE0, "uge")
///     .charset(Charset::ASCII)
///     .build()
///     .unwrap();
/// assert_eq!(request.method(), Method::GET);
/// ```
#[derive(Debug)]
pub struct Request {
    pub(crate) method: Method,
    pub(crate) version: Version,
    pub(crate) headers: HeaderMap,
    pub(crate) charset: Charset,
}

impl Request {
    /// Parse a bytes into a Request.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uka_shiori::types::v3::{Request, Method};
    /// #
    /// let input = [
    ///     b"GET SHIORI/3.0\r\n".to_vec(),
    ///     b"Sender: Materia\r\n".to_vec(),
    ///     b"ID: hoge\r\n".to_vec(),
    ///     b"Reference0: uge\r\n".to_vec(),
    ///     b"Charset: UTF-8\r\n".to_vec(),
    ///     b"\r\n".to_vec()
    /// ].concat();
    /// let request = Request::parse(&input).unwrap();
    /// assert_eq!(request.method(), Method::GET);
    /// ```
    pub fn parse(buf: &[u8]) -> Result<Self, ParseError> {
        parse_request(buf)
    }

    /// Returns a builder that generates a type for the Request
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uka_shiori::types::v3::{Request, Method, Version, HeaderName, Charset};
    /// #
    /// let request = Request::builder()
    ///    .method(Method::GET)
    ///    .version(Version::SHIORI_30)
    ///    .header(HeaderName::SENDER, "Materia")
    ///    .header(HeaderName::ID, "hoge")
    ///    .header(HeaderName::REFERENCE0, "uge")
    ///    .charset(Charset::ASCII)
    ///    .build()
    ///    .unwrap();
    /// assert_eq!(request.method(), Method::GET);
    /// ```
    pub fn builder() -> RequestBuilder {
        RequestBuilder::new()
    }

    /// Returns SHIORI method.
    pub fn method(&self) -> Method {
        self.method
    }

    /// Returns SHIORI version.
    pub fn version(&self) -> Version {
        self.version
    }

    /// Returns SHIORI header fields.
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    /// Returns SHIORI charset.
    pub fn charset(&self) -> Charset {
        self.charset
    }

    /// Returns sender in SHIORI header fields.
    pub fn sender(&self) -> Option<&HeaderValue> {
        self.headers.get(&HeaderName::SENDER)
    }

    /// Returns ID in SHIORI header fields.
    pub fn id(&self) -> Option<&HeaderValue> {
        self.headers.get(&HeaderName::ID)
    }

    /// Returns Reference0 in SHIORI header fields.
    pub fn reference0(&self) -> Option<&HeaderValue> {
        self.headers.get(&HeaderName::REFERENCE0)
    }

    /// Returns Reference1 in SHIORI header fields.
    pub fn reference1(&self) -> Option<&HeaderValue> {
        self.headers.get(&HeaderName::REFERENCE1)
    }

    /// Returns Reference2 in SHIORI header fields.
    pub fn reference2(&self) -> Option<&HeaderValue> {
        self.headers.get(&HeaderName::REFERENCE2)
    }

    /// Returns Reference3 in SHIORI header fields.
    pub fn reference3(&self) -> Option<&HeaderValue> {
        self.headers.get(&HeaderName::REFERENCE3)
    }

    /// Returns Reference4 in SHIORI header fields.
    pub fn reference4(&self) -> Option<&HeaderValue> {
        self.headers.get(&HeaderName::REFERENCE4)
    }

    /// Returns Reference5 in SHIORI header fields.
    pub fn reference5(&self) -> Option<&HeaderValue> {
        self.headers.get(&HeaderName::REFERENCE5)
    }

    /// Returns Reference6 in SHIORI header fields.
    pub fn reference6(&self) -> Option<&HeaderValue> {
        self.headers.get(&HeaderName::REFERENCE6)
    }

    /// Returns Reference7 in SHIORI header fields.
    pub fn reference7(&self) -> Option<&HeaderValue> {
        self.headers.get(&HeaderName::REFERENCE7)
    }

    /// Returns SecurityLevel in SHIORI header fields.
    pub fn security_level(&self) -> Option<&HeaderValue> {
        self.headers.get(&HeaderName::SECURITY_LEVEL)
    }

    /// Convert request to bytes.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(self.method.to_string().as_bytes());
        buf.extend_from_slice(b" ");
        buf.extend_from_slice(self.version.to_string().as_bytes());
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

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Request::parse(value)
    }
}

impl TryFrom<Vec<u8>> for Request {
    type Error = ParseError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Request::parse(&value)
    }
}

/// Error that can occur when build SHIORI request.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("method is required")]
    MissingMethod,
    #[error("version is required")]
    MissingVersion,
    #[error("charset is required")]
    MissingCharset,
    #[error("{0}")]
    InvalidHeaderName(#[from] HeaderNameError),
    #[error("{0}")]
    FailedEncodeHeaderValue(#[from] HeaderValueError),
}

#[derive(Default)]
struct Parts {
    method: Option<Method>,
    version: Option<Version>,
    headers: OrderedBag<String, String>,
    charset: Option<Charset>,
}

/// Builder for SHIORI v3 request.
pub struct RequestBuilder {
    inner: Result<Parts, Error>,
}

impl RequestBuilder {
    pub(crate) fn new() -> Self {
        Self {
            inner: Ok(Parts::default()),
        }
    }

    /// Build NOTIFY request.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uka_shiori::types::v3::{RequestBuilder, Method, Version, HeaderName, Charset};
    /// #
    /// let request = RequestBuilder::notify(Version::SHIORI_30)
    ///   .header(HeaderName::SENDER, "Materia")
    ///   .header(HeaderName::ID, "OnInitialize")
    ///   .header(HeaderName::REFERENCE0, "Reference0")
    ///   .charset(Charset::ASCII)
    ///   .build()
    ///   .unwrap();
    /// assert_eq!(request.method(), Method::NOTIFY);
    /// ```
    pub fn notify(version: Version) -> Self {
        RequestBuilder::new()
            .version(version)
            .method(Method::NOTIFY)
    }

    /// Build GET request.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uka_shiori::types::v3::{RequestBuilder, Method, Version, HeaderName, Charset};
    /// #
    /// let request = RequestBuilder::get(Version::SHIORI_30)
    ///   .header(HeaderName::SENDER, "Materia")
    ///   .header(HeaderName::ID, "version")
    ///   .charset(Charset::ASCII)
    ///   .build()
    ///   .unwrap();
    /// assert_eq!(request.method(), Method::GET);
    /// ```
    pub fn get(version: Version) -> Self {
        RequestBuilder::new().version(version).method(Method::GET)
    }

    /// Set SHIORI method.
    pub fn method(self, method: Method) -> Self {
        self.and_then(|inner| {
            Ok(Parts {
                method: Some(method),
                ..inner
            })
        })
    }

    /// Set SHIORI version.
    pub fn version(self, version: Version) -> Self {
        self.and_then(|inner| {
            Ok(Parts {
                version: Some(version),
                ..inner
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
            inner.headers.insert(name.into(), value.into());
            Ok(inner)
        })
    }

    /// Set SHIORI charset.
    pub fn charset(self, charset: Charset) -> Self {
        self.and_then(|mut inner| {
            inner
                .headers
                .insert(HeaderName::CHARSET.to_string(), charset.to_string());
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

    /// Build SHIORI request.
    pub fn build(self) -> Result<Request, Error> {
        let inner = self.inner?;
        let charset = inner.charset.unwrap_or(Charset::ASCII);
        Ok(Request {
            method: inner.method.ok_or(Error::MissingMethod)?,
            version: inner.version.ok_or(Error::MissingVersion)?,
            headers: inner
                .headers
                .into_iter()
                .map(|(k, v)| {
                    HeaderName::from_static(&k)
                        .map_err(Error::InvalidHeaderName)
                        .and_then(|name| {
                            HeaderValue::from_static_with_charset(&v, charset)
                                .map(|value| (name, value))
                                .map_err(Error::FailedEncodeHeaderValue)
                        })
                })
                .collect::<Result<HeaderMap, Error>>()?,

            charset,
        })
    }

    fn and_then<F>(self, func: F) -> Self
    where
        F: FnOnce(Parts) -> Result<Parts, Error>,
    {
        RequestBuilder {
            inner: self.inner.and_then(func),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_parse_and_builder_will_be_same() -> anyhow::Result<()> {
        let input = [
            b"GET SHIORI/3.0\r\n".to_vec(),
            b"Sender: Materia\r\n".to_vec(),
            b"ID: hoge\r\n".to_vec(),
            b"Reference0: uge\r\n".to_vec(),
            b"\r\n".to_vec(),
        ]
        .concat();
        let request1 = Request::parse(&input)?;
        let request2 = Request::builder()
            .version(Version::SHIORI_30)
            .method(Method::GET)
            .header(HeaderName::SENDER, "Materia")
            .header(HeaderName::ID, "hoge")
            .header(HeaderName::REFERENCE0, "uge")
            .build()?;

        assert_eq!(request1.method(), request2.method());
        assert_eq!(request1.version(), request2.version());
        assert_eq!(request1.charset(), request2.charset());
        assert_eq!(request1.sender(), request2.sender());
        assert_eq!(request1.id(), request2.id());
        assert_eq!(request1.reference0(), request2.reference0());
        assert_eq!(request1.reference1(), request2.reference1());
        assert_eq!(request1.reference2(), request2.reference2());
        assert_eq!(request1.reference3(), request2.reference3());
        assert_eq!(request1.reference4(), request2.reference4());
        assert_eq!(request1.reference5(), request2.reference5());
        assert_eq!(request1.reference6(), request2.reference6());
        assert_eq!(request1.reference7(), request2.reference7());
        assert_eq!(request1.security_level(), request2.security_level());

        assert_eq!(
            request1.as_bytes(),
            request2.as_bytes(),
            "\nassertion failed: `(left == right)\n  left: `{:?}`,\n right: `{:?}`",
            String::from_utf8_lossy(&request1.as_bytes()),
            String::from_utf8_lossy(&request2.as_bytes())
        );

        Ok(())
    }
}
