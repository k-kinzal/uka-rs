use crate::charset::Charset;
use crate::header::{HeaderMap, HeaderName, HeaderNameError, HeaderValue, HeaderValueError};
use crate::method::Method;
use crate::parse;
use crate::parse::parse_request;
use crate::version::Version;
use std::collections::HashMap;

/// Request is a type that represents an SSTP request.
///
/// Request provides a builder to generate types, a parser to generate types from bytes,
/// and an accessor to the headers defined in the specification.
///
/// ```rust
/// # use uka_sstp::{Charset, HeaderName, Method, Version};
/// # use uka_sstp::request::Request;
/// let request = Request::builder()
///     .send(Version::SSTP_11)
///     .header(HeaderName::SENDER, "カードキャプター")
///     .header(HeaderName::SCRIPT, "\\h\\s0汝のあるべき姿に戻れ。\\e")
///     .header(HeaderName::OPTION, "nodescript,notranslate")
///     .charset(Charset::SHIFT_JIS)
///     .build()
///     .unwrap();
/// assert_eq!(request.method(), Method::SEND);
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
    /// ```rust
    /// # use uka_util::encode::Encoder;
    /// # use uka_sstp::{Charset, HeaderName, Method, Version};
    /// # use uka_sstp::request::Request;
    /// let input = [
    ///     b"SEND SSTP/1.1\r\n".to_vec(),
    ///     b"Sender:".to_vec(),
    ///     Encoder::encode_sjis("カードキャプター").unwrap(),
    ///     b"\r\n".to_vec(),
    ///     b"Script:".to_vec(),
    ///     Encoder::encode_sjis("\\h\\s0汝のあるべき姿に戻れ。\\e").unwrap(),
    ///     b"\r\n".to_vec(),
    ///     b"Option:nodescript,notranslate\r\n".to_vec(),
    ///     b"Charset:Shift_JIS\r\n".to_vec(),
    ///     b"\r\n".to_vec(),
    /// ].concat();
    /// let request = Request::parse(&input).unwrap();
    /// assert_eq!(request.method(), Method::SEND);
    /// ```
    pub fn parse(buf: &[u8]) -> Result<Self, parse::Error> {
        parse_request(buf)
    }

    /// Returns a builder that generates a type for the Request
    ///
    /// ```rust
    /// # use uka_sstp::{Charset, HeaderName, Method, Version};
    /// # use uka_sstp::request::Request;
    /// let request = Request::builder()
    ///     .notify(Version::SSTP_10)
    ///     .header(HeaderName::SENDER, "さくら")
    ///     .header(HeaderName::EVENT, "OnMusicPlay")
    ///     .header(HeaderName::REFERENCE0, "元祖高木ブー伝説")
    ///     .header(HeaderName::REFERENCE1, "筋肉少女帯")
    ///     .charset(Charset::SHIFT_JIS)
    ///     .build()
    ///     .unwrap();
    /// assert_eq!(request.method(), Method::NOTIFY);
    /// ```
    pub fn builder() -> Builder {
        Builder::new()
    }

    /// Returns SSTP method.
    pub fn method(&self) -> Method {
        self.method
    }

    /// Returns SSTP version.
    pub fn version(&self) -> Version {
        self.version
    }

    /// Returns SSTP header fields.
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    /// Returns SSTP charset.
    pub fn charset(&self) -> Charset {
        self.charset
    }

    /// Returns sender in SSTP header fields.
    pub fn sender(&self) -> Option<&HeaderValue> {
        self.headers.get(&HeaderName::SENDER)
    }

    /// Returns event in SSTP header fields.
    pub fn event(&self) -> Option<&HeaderValue> {
        self.headers.get(&HeaderName::EVENT)
    }

    /// Returns reference0 in SSTP header fields.
    pub fn reference0(&self) -> Option<&HeaderValue> {
        self.headers.get(&HeaderName::REFERENCE0)
    }

    /// Returns reference1 in SSTP header fields.
    pub fn reference1(&self) -> Option<&HeaderValue> {
        self.headers.get(&HeaderName::REFERENCE1)
    }

    /// Returns reference2 in SSTP header fields.
    pub fn reference2(&self) -> Option<&HeaderValue> {
        self.headers.get(&HeaderName::REFERENCE2)
    }

    /// Returns reference3 in SSTP header fields.
    pub fn reference3(&self) -> Option<&HeaderValue> {
        self.headers.get(&HeaderName::REFERENCE3)
    }

    /// Returns reference4 in SSTP header fields.
    pub fn reference4(&self) -> Option<&HeaderValue> {
        self.headers.get(&HeaderName::REFERENCE4)
    }

    /// Returns reference5 in SSTP header fields.
    pub fn reference5(&self) -> Option<&HeaderValue> {
        self.headers.get(&HeaderName::REFERENCE5)
    }

    /// Returns reference6 in SSTP header fields.
    pub fn reference6(&self) -> Option<&HeaderValue> {
        self.headers.get(&HeaderName::REFERENCE6)
    }

    /// Returns reference7 in SSTP header fields.
    pub fn reference7(&self) -> Option<&HeaderValue> {
        self.headers.get(&HeaderName::REFERENCE7)
    }

    /// Returns script in SSTP header fields.
    pub fn script(&self) -> Vec<&HeaderValue> {
        self.headers.get_all(&HeaderName::SCRIPT)
    }

    /// Returns option in SSTP header fields.
    pub fn option(&self) -> Option<&HeaderValue> {
        self.headers.get(&HeaderName::OPTION)
    }

    /// Returns entry in SSTP header fields.
    pub fn entry(&self) -> Vec<&HeaderValue> {
        self.headers.get_all(&HeaderName::ENTRY)
    }

    /// Returns hwnd in SSTP header fields.
    pub fn hwnd(&self) -> Option<&HeaderValue> {
        self.headers.get(&HeaderName::HWND)
    }

    /// Returns if_ghost in SSTP header fields.
    pub fn if_ghost(&self) -> Vec<&HeaderValue> {
        self.headers.get_all(&HeaderName::IF_GHOST)
    }

    /// Returns command in SSTP header fields.
    pub fn command(&self) -> Option<&HeaderValue> {
        self.headers.get(&HeaderName::COMMAND)
    }

    /// Returns document in SSTP header fields.
    pub fn document(&self) -> Option<&HeaderValue> {
        self.headers.get(&HeaderName::DOCUMENT)
    }

    /// Returns songname in SSTP header fields.
    pub fn songname(&self) -> Option<&HeaderValue> {
        self.headers.get(&HeaderName::SONGNAME)
    }

    /// Returns sentence in SSTP header fields.
    pub fn sentence(&self) -> Option<&HeaderValue> {
        self.headers.get(&HeaderName::SENTENCE)
    }

    /// Returns port in SSTP header fields.
    pub fn port(&self) -> Option<&HeaderValue> {
        self.headers.get(&HeaderName::PORT)
    }

    /// Returns surface in SSTP header fields.
    pub fn surface(&self) -> Option<&HeaderValue> {
        self.headers.get(&HeaderName::SURFACE)
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

/// Error that can occur when build SSTP request.
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
    headers: HashMap<String, Vec<String>>,
    charset: Option<Charset>,
}

/// Builder for SSTP request.
pub struct Builder {
    inner: Result<Parts, Error>,
}

impl Builder {
    pub(crate) fn new() -> Self {
        Self {
            inner: Ok(Parts::default()),
        }
    }

    /// Build notify request.
    ///
    /// ```rust
    /// # use uka_sstp::{Charset, HeaderName, Method, Version};
    /// # use uka_sstp::request::Request;
    /// let builder = Request::builder();
    /// let request = builder
    ///     .notify(Version::SSTP_10)
    ///     .header(HeaderName::SENDER, "さくら")
    ///     .header(HeaderName::EVENT, "OnMusicPlay")
    ///     .header(HeaderName::REFERENCE0, "元祖高木ブー伝説")
    ///     .header(HeaderName::REFERENCE1, "筋肉少女帯")
    ///     .charset(Charset::SHIFT_JIS)
    ///     .build()
    ///     .unwrap();
    /// assert_eq!(request.method(), Method::NOTIFY);
    /// ```
    pub fn notify(self, version: Version) -> Self {
        self.method(Method::NOTIFY).version(version)
    }

    /// Build send request.
    ///
    /// ```rust
    /// # use uka_sstp::{Charset, HeaderName, Method, Version};
    /// # use uka_sstp::request::Request;
    /// let builder = Request::builder();
    /// let request = builder
    ///     .send(Version::SSTP_11)
    ///     .header(HeaderName::SENDER, "カードキャプター")
    ///     .header(HeaderName::SCRIPT, "\\h\\s0汝のあるべき姿に戻れ。\\e")
    ///     .header(HeaderName::OPTION, "nodescript,notranslate")
    ///     .charset(Charset::SHIFT_JIS)
    ///     .build()
    ///     .unwrap();
    /// assert_eq!(request.method(), Method::SEND);
    /// ```
    pub fn send(self, version: Version) -> Self {
        self.method(Method::SEND).version(version)
    }

    /// Build execute request.
    ///
    /// ```rust
    /// # use uka_sstp::{Charset, HeaderName, Method, Version};
    /// # use uka_sstp::request::Request;
    /// let builder = Request::builder();
    /// let request = builder
    ///     .execute(Version::SSTP_10)
    ///     .header(HeaderName::SENDER, "サンプルプログラム")
    ///     .header(HeaderName::COMMAND, "GetName")
    ///     .charset(Charset::SHIFT_JIS)
    ///     .build()
    ///     .unwrap();
    /// assert_eq!(request.method(), Method::EXECUTE);
    /// ```
    pub fn execute(self, version: Version) -> Self {
        self.method(Method::EXECUTE).version(version)
    }

    /// Build give request.
    ///
    /// ```rust
    /// # use uka_sstp::{Charset, HeaderName, Method, Version};
    /// # use uka_sstp::request::Request;
    /// let builder = Request::builder();
    /// let request = builder
    ///     .give(Version::SSTP_11)
    ///     .header(HeaderName::SENDER, "カードキャプター")
    ///     .header(HeaderName::DOCUMENT, "こんにちはさくらです。闇の力を秘めし鍵よ真の姿を我の前に示せレリーズ。汝のあるべき姿に戻れクロウカード。")
    ///     .charset(Charset::SHIFT_JIS)
    ///     .build()
    ///     .unwrap();
    /// assert_eq!(request.method(), Method::GIVE);
    /// ```
    pub fn give(self, version: Version) -> Self {
        self.method(Method::GIVE).version(version)
    }

    /// Build communicate request.
    ///
    /// ```rust
    /// # use uka_sstp::{Charset, HeaderName, Method, Version};
    /// # use uka_sstp::request::Request;
    /// let builder = Request::builder();
    /// let request = builder
    ///     .communicate(Version::SSTP_11)
    ///     .header(HeaderName::SENDER, "カードキャプター")
    ///     .header(HeaderName::SENTENCE, "今日は寒いなー。")
    ///     .header(HeaderName::OPTION, "substitute")
    ///     .charset(Charset::SHIFT_JIS)
    ///     .build()
    ///     .unwrap();
    /// assert_eq!(request.method(), Method::COMMUNICATE);
    /// ```
    pub fn communicate(self, version: Version) -> Self {
        self.method(Method::COMMUNICATE).version(version)
    }

    /// Set SSTP method.
    ///
    /// ```rust
    /// # use uka_sstp::{Charset, HeaderName, Method, Version};
    /// # use uka_sstp::request::Request;
    /// let builder = Request::builder();
    /// let request = builder
    ///     .method(Method::NOTIFY)
    ///     .version(Version::SSTP_10)
    ///     .header(HeaderName::SENDER, "さくら")
    ///     .header(HeaderName::EVENT, "OnMusicPlay")
    ///     .header(HeaderName::REFERENCE0, "元祖高木ブー伝説")
    ///     .header(HeaderName::REFERENCE1, "筋肉少女帯")
    ///     .charset(Charset::SHIFT_JIS)
    ///     .build()
    ///     .unwrap();
    /// assert_eq!(request.method(), Method::NOTIFY);
    /// ```
    pub fn method(self, method: Method) -> Self {
        self.and_then(|inner| {
            Ok(Parts {
                method: Some(method),
                ..inner
            })
        })
    }

    /// Set SSTP version.
    pub fn version(self, version: Version) -> Self {
        self.and_then(|inner| {
            Ok(Parts {
                version: Some(version),
                ..inner
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

    /// Build SSTP request.
    pub fn build(self) -> Result<Request, Error> {
        let inner = self.inner?;
        let charset = inner.charset.ok_or(Error::MissingCharset)?;
        Ok(Request {
            method: inner.method.ok_or(Error::MissingMethod)?,
            version: inner.version.ok_or(Error::MissingVersion)?,
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
