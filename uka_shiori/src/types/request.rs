use crate::types::v3;

#[derive(thiserror::Error, Debug)]
pub enum RequestParseError {
    #[error("failed parse: invalid request")]
    InvalidRequest,

    #[error("{0}")]
    V3(#[from] v3::ParseError),
}

/// `Request` is a type that represents a SHIORI request.
///
/// This type supports multiple versions of SHIORI.
/// If you want to support only a specific version, use `uka_shiori::types::v3::Request` and a specific version explicitly.
pub enum Request {
    V3(v3::Request),
}

impl Request {
    /// Parse a bytes into a Request.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uka_shiori::types::Request;
    /// # use uka_shiori::types::v3::Version;
    /// #
    /// let input = [
    ///     b"GET SHIORI/3.0\r\n".to_vec(),
    ///     b"Sender: Materia\r\n".to_vec(),
    ///     b"ID: hoge\r\n".to_vec(),
    ///     b"Reference0: uge\r\n".to_vec(),
    ///     b"Charset: UTF-8\r\n".to_vec(),
    ///     b"\r\n".to_vec()
    /// ].concat();
    ///
    /// match Request::parse(&input) {
    ///     Ok(Request::V3(request)) => {
    ///        assert_eq!(request.version(), Version::SHIORI_30);
    ///     },
    ///     Err(e) => panic!("{e}"),
    /// }
    /// ```
    pub fn parse(buf: &[u8]) -> Result<Self, RequestParseError> {
        #[allow(clippy::if_same_then_else)]
        if b"GET SHIORI/3.0" == &buf[..14] || b"NOTIFY SHIORI/3.0" == &buf[..17] {
            // parse as SHIORI/3.0
            Ok(Request::V3(
                v3::Request::parse(buf).map_err(RequestParseError::from)?,
            ))
        } else if b"GET" == &buf[..3] || b"NOTIFY" == &buf[..6] || b"TEACH" == &buf[..5] {
            // parse as SHIORI/2.x
            Err(RequestParseError::InvalidRequest)
        } else {
            // parse as SHIORI/1.x
            Err(RequestParseError::InvalidRequest)
        }
    }

    /// Convert request to bytes.
    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            Self::V3(request) => request.as_bytes(),
        }
    }
}

impl From<v3::Request> for Request {
    fn from(value: v3::Request) -> Self {
        Self::V3(value)
    }
}

/// `RequestExt` is a trait that extends the `Request` type.
pub trait RequestExt<V, R> {
    /// Returns a concrete request builder by specifying a version.
    fn builder(version: V) -> R;
}

impl RequestExt<v3::Version, v3::RequestBuilder> for Request {
    /// Returns a concrete request builder by specifying a version.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uka_shiori::types::{Request, RequestExt, v3};
    /// #
    /// let request = Request::builder(v3::Version::SHIORI_30)
    ///   .method(v3::Method::GET)
    ///   .header(v3::HeaderName::SENDER, "Materia")
    ///   .header(v3::HeaderName::ID, "version")
    ///   .charset(v3::Charset::UTF8)
    ///   .build()
    ///   .unwrap();
    /// assert_eq!(request.version(), v3::Version::SHIORI_30);
    /// ```
    fn builder(version: v3::Version) -> v3::RequestBuilder {
        v3::RequestBuilder::new().version(version)
    }
}
