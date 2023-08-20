use std::fmt::Display;
use uka_util::string::{Error as Rfc7230StringConvertError, Rfc7230String};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Inner {
    Charset,
    Sender,
    ID,
    Reference0,
    Reference1,
    Reference2,
    Reference3,
    Reference4,
    Reference5,
    Reference6,
    Reference7,
    SecurityLevel,
    Value,
    Other(Rfc7230String),
}

/// Error that can occur when convert from string.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid header name: {0}")]
    InvalidHeaderName(#[from] Rfc7230StringConvertError),
}

/// HeaderName is the name of the SHIORI header field.
///
/// It has some field names defined based on SHIORI specifications and extended proprietary field names.
/// HeaderName is used as a key in the HeaderMap; constants are available for header names based on the SHIORI specification.
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct HeaderName(Inner);

impl HeaderName {
    /// Charset
    pub const CHARSET: HeaderName = HeaderName(Inner::Charset);

    /// Sender
    pub const SENDER: HeaderName = HeaderName(Inner::Sender);

    /// ID
    pub const ID: HeaderName = HeaderName(Inner::ID);

    /// Reference0
    pub const REFERENCE0: HeaderName = HeaderName(Inner::Reference0);

    /// Reference1
    pub const REFERENCE1: HeaderName = HeaderName(Inner::Reference1);

    /// Reference2
    pub const REFERENCE2: HeaderName = HeaderName(Inner::Reference2);

    /// Reference3
    pub const REFERENCE3: HeaderName = HeaderName(Inner::Reference3);

    /// Reference4
    pub const REFERENCE4: HeaderName = HeaderName(Inner::Reference4);

    /// Reference5
    pub const REFERENCE5: HeaderName = HeaderName(Inner::Reference5);

    /// Reference6
    pub const REFERENCE6: HeaderName = HeaderName(Inner::Reference6);

    /// Reference7
    pub const REFERENCE7: HeaderName = HeaderName(Inner::Reference7);

    /// SecurityLevel
    pub const SECURITY_LEVEL: HeaderName = HeaderName(Inner::SecurityLevel);

    /// Value
    pub const VALUE: HeaderName = HeaderName(Inner::Value);

    ///　Converts a str to HeaderName.
    ///
    /// ```rust
    /// # use uka_shiori::types::v3::HeaderName;
    /// assert_eq!(HeaderName::from_static("Charset").unwrap(), HeaderName::CHARSET);
    /// assert_eq!(HeaderName::from_static("Sender").unwrap(), HeaderName::SENDER);
    /// assert_eq!(HeaderName::from_static("ID").unwrap(), HeaderName::ID);
    /// assert_eq!(HeaderName::from_static("Reference0").unwrap(), HeaderName::REFERENCE0);
    /// assert_eq!(HeaderName::from_static("Reference1").unwrap(), HeaderName::REFERENCE1);
    /// assert_eq!(HeaderName::from_static("Reference2").unwrap(), HeaderName::REFERENCE2);
    /// assert_eq!(HeaderName::from_static("Reference3").unwrap(), HeaderName::REFERENCE3);
    /// assert_eq!(HeaderName::from_static("Reference4").unwrap(), HeaderName::REFERENCE4);
    /// assert_eq!(HeaderName::from_static("Reference5").unwrap(), HeaderName::REFERENCE5);
    /// assert_eq!(HeaderName::from_static("Reference6").unwrap(), HeaderName::REFERENCE6);
    /// assert_eq!(HeaderName::from_static("Reference7").unwrap(), HeaderName::REFERENCE7);
    /// assert_eq!(HeaderName::from_static("SecurityLevel").unwrap(), HeaderName::SECURITY_LEVEL);
    /// assert_eq!(HeaderName::from_static("Value").unwrap(), HeaderName::VALUE);
    /// ```
    pub fn from_static(s: &str) -> Result<HeaderName, Error> {
        match s {
            "Charset" => Ok(HeaderName(Inner::Charset)),
            "Sender" => Ok(HeaderName(Inner::Sender)),
            "ID" => Ok(HeaderName(Inner::ID)),
            "Reference0" => Ok(HeaderName(Inner::Reference0)),
            "Reference1" => Ok(HeaderName(Inner::Reference1)),
            "Reference2" => Ok(HeaderName(Inner::Reference2)),
            "Reference3" => Ok(HeaderName(Inner::Reference3)),
            "Reference4" => Ok(HeaderName(Inner::Reference4)),
            "Reference5" => Ok(HeaderName(Inner::Reference5)),
            "Reference6" => Ok(HeaderName(Inner::Reference6)),
            "Reference7" => Ok(HeaderName(Inner::Reference7)),
            "SecurityLevel" => Ok(HeaderName(Inner::SecurityLevel)),
            "Value" => Ok(HeaderName(Inner::Value)),
            _ => Ok(HeaderName(Inner::Other(Rfc7230String::from_string(
                s.to_string(),
            )?))),
        }
    }

    ///　Converts a bytes to HeaderName.
    ///
    /// ```rust
    /// # use uka_shiori::types::v3::HeaderName;
    /// assert_eq!(HeaderName::from_bytes(b"Charset").unwrap(), HeaderName::CHARSET);
    /// assert_eq!(HeaderName::from_bytes(b"Sender").unwrap(), HeaderName::SENDER);
    /// assert_eq!(HeaderName::from_bytes(b"ID").unwrap(), HeaderName::ID);
    /// assert_eq!(HeaderName::from_bytes(b"Reference0").unwrap(), HeaderName::REFERENCE0);
    /// assert_eq!(HeaderName::from_bytes(b"Reference1").unwrap(), HeaderName::REFERENCE1);
    /// assert_eq!(HeaderName::from_bytes(b"Reference2").unwrap(), HeaderName::REFERENCE2);
    /// assert_eq!(HeaderName::from_bytes(b"Reference3").unwrap(), HeaderName::REFERENCE3);
    /// assert_eq!(HeaderName::from_bytes(b"Reference4").unwrap(), HeaderName::REFERENCE4);
    /// assert_eq!(HeaderName::from_bytes(b"Reference5").unwrap(), HeaderName::REFERENCE5);
    /// assert_eq!(HeaderName::from_bytes(b"Reference6").unwrap(), HeaderName::REFERENCE6);
    /// assert_eq!(HeaderName::from_bytes(b"Reference7").unwrap(), HeaderName::REFERENCE7);
    /// assert_eq!(HeaderName::from_bytes(b"SecurityLevel").unwrap(), HeaderName::SECURITY_LEVEL);
    /// assert_eq!(HeaderName::from_bytes(b"Value").unwrap(), HeaderName::VALUE);
    /// ```
    pub fn from_bytes(bytes: &[u8]) -> Result<HeaderName, Error> {
        let s = String::from_utf8_lossy(bytes);
        HeaderName::from_static(s.as_ref())
    }

    ///　Converts a HeaderName to bytes.
    ///
    /// ```rust
    /// # use uka_shiori::types::v3::HeaderName;
    /// assert_eq!(HeaderName::CHARSET.to_vec(), b"Charset");
    /// assert_eq!(HeaderName::SENDER.to_vec(), b"Sender");
    /// assert_eq!(HeaderName::ID.to_vec(), b"ID");
    /// assert_eq!(HeaderName::REFERENCE0.to_vec(), b"Reference0");
    /// assert_eq!(HeaderName::REFERENCE1.to_vec(), b"Reference1");
    /// assert_eq!(HeaderName::REFERENCE2.to_vec(), b"Reference2");
    /// assert_eq!(HeaderName::REFERENCE3.to_vec(), b"Reference3");
    /// assert_eq!(HeaderName::REFERENCE4.to_vec(), b"Reference4");
    /// assert_eq!(HeaderName::REFERENCE5.to_vec(), b"Reference5");
    /// assert_eq!(HeaderName::REFERENCE6.to_vec(), b"Reference6");
    /// assert_eq!(HeaderName::REFERENCE7.to_vec(), b"Reference7");
    /// assert_eq!(HeaderName::SECURITY_LEVEL.to_vec(), b"SecurityLevel");
    /// assert_eq!(HeaderName::VALUE.to_vec(), b"Value");
    /// assert_eq!(HeaderName::from_static("X-Extend-Header").unwrap().to_vec(), b"X-Extend-Header");
    /// ```
    pub fn to_vec(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}

impl Display for HeaderName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Inner::Charset => write!(f, "Charset"),
            Inner::Sender => write!(f, "Sender"),
            Inner::ID => write!(f, "ID"),
            Inner::Reference0 => write!(f, "Reference0"),
            Inner::Reference1 => write!(f, "Reference1"),
            Inner::Reference2 => write!(f, "Reference2"),
            Inner::Reference3 => write!(f, "Reference3"),
            Inner::Reference4 => write!(f, "Reference4"),
            Inner::Reference5 => write!(f, "Reference5"),
            Inner::Reference6 => write!(f, "Reference6"),
            Inner::Reference7 => write!(f, "Reference7"),
            Inner::SecurityLevel => write!(f, "SecurityLevel"),
            Inner::Value => write!(f, "Value"),
            Inner::Other(s) => write!(f, "{s}"),
        }
    }
}

impl From<HeaderName> for String {
    fn from(header_name: HeaderName) -> Self {
        header_name.to_string()
    }
}
