use crate::types::v3;

#[derive(thiserror::Error, Debug)]
pub enum ResponseParseError {
    #[error("failed parse: invalid response")]
    InvalidResponse,

    #[error("{0}")]
    V3(#[from] v3::ParseError),
}

/// `Response` is a type that represents a SHIORI request.
///
/// This type supports multiple versions of SHIORI.
/// If you want to support only a specific version, use `uka_shiori::types::v3::Response` and a specific version explicitly.
#[derive(Debug)]
pub enum Response {
    V3(v3::Response),
}

impl Response {
    /// Parse a bytes into a Request.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uka_shiori::types::Response;
    /// # use uka_shiori::types::v3::Version;
    /// #
    /// let input = [
    ///     b"SHIORI/3.0 204 No Content\r\n".to_vec(),
    ///     b"Sender: F.I.R.S.T\r\n".to_vec(),
    ///     b"Value: hoge\r\n".to_vec(),
    ///     b"Charset: UTF-8\r\n".to_vec(),
    ///     b"\r\n".to_vec(),
    /// ].concat();
    ///
    /// match Response::parse(&input) {
    ///     Ok(Response::V3(response)) => {
    ///       assert_eq!(response.version(), Version::SHIORI_30);
    ///     },
    ///     Err(e) => panic!("{e}"),
    /// }
    /// ```
    pub fn parse(buf: &[u8]) -> Result<Self, ResponseParseError> {
        #[allow(clippy::if_same_then_else)]
        if b"SHIORI/3" == buf[..8].as_ref() {
            // parse as SHIORI/3.0
            Ok(Response::V3(v3::Response::parse(buf)?))
        } else if b"SHIORI/2" == buf[..8].as_ref() {
            // parse as SHIORI/2.x
            Err(ResponseParseError::InvalidResponse)
        } else {
            // parse as SHIORI/1.x
            Err(ResponseParseError::InvalidResponse)
        }
    }

    /// Convert response to bytes.
    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            Self::V3(request) => request.as_bytes(),
        }
    }
}

impl From<v3::Response> for Response {
    fn from(value: v3::Response) -> Self {
        Self::V3(value)
    }
}

/// `ResponseExt` is a trait that extends the `Response` type.
pub trait ResponseExt<V, R> {
    /// Returns a concrete response builder by specifying a version.
    fn builder(version: V) -> R;
}

impl ResponseExt<v3::Version, v3::ResponseBuilder> for Response {
    /// Returns a concrete response builder by specifying a version.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uka_shiori::types::{Response, ResponseExt};
    /// # use uka_shiori::types::v3::{Charset, HeaderName, StatusCode, Version};
    /// #
    /// let response = Response::builder(Version::SHIORI_30)
    ///   .status_code(StatusCode::NO_CONTENT)
    ///   .header(HeaderName::SENDER, "F.I.R.S.T")
    ///   .header(HeaderName::VALUE, "hoge")
    ///   .charset(Charset::UTF8)
    ///   .build()
    ///   .unwrap();
    /// assert_eq!(response.version(), Version::SHIORI_30);
    /// ```
    fn builder(version: v3::Version) -> v3::ResponseBuilder {
        v3::ResponseBuilder::new().version(version)
    }
}
