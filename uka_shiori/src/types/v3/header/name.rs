use std::fmt::Display;

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
    Other(String),
}

/// Error that can occur when convert from string.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid header name: {0}")]
    InvalidHeaderName(String),
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
            _ => {
                let valid = s.as_bytes().iter().all(|byte| matches!(*byte, b'0'..=b'9' | b'!' | b'#'..=b'\'' | b'*'..=b'+' | b'-'..=b'.' | b'^'..=b'`' | b'A'..=b'Z' | b'a'..=b'z' | b'|' | b'~'));
                if valid {
                    Ok(HeaderName(Inner::Other(s.to_string())))
                } else {
                    Err(Error::InvalidHeaderName(s.to_string()))
                }
            }
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
    /// assert_eq!(HeaderName::CHARSET.as_bytes(), b"Charset");
    /// assert_eq!(HeaderName::SENDER.as_bytes(), b"Sender");
    /// assert_eq!(HeaderName::ID.as_bytes(), b"ID");
    /// assert_eq!(HeaderName::REFERENCE0.as_bytes(), b"Reference0");
    /// assert_eq!(HeaderName::REFERENCE1.as_bytes(), b"Reference1");
    /// assert_eq!(HeaderName::REFERENCE2.as_bytes(), b"Reference2");
    /// assert_eq!(HeaderName::REFERENCE3.as_bytes(), b"Reference3");
    /// assert_eq!(HeaderName::REFERENCE4.as_bytes(), b"Reference4");
    /// assert_eq!(HeaderName::REFERENCE5.as_bytes(), b"Reference5");
    /// assert_eq!(HeaderName::REFERENCE6.as_bytes(), b"Reference6");
    /// assert_eq!(HeaderName::REFERENCE7.as_bytes(), b"Reference7");
    /// assert_eq!(HeaderName::SECURITY_LEVEL.as_bytes(), b"SecurityLevel");
    /// assert_eq!(HeaderName::VALUE.as_bytes(), b"Value");
    /// assert_eq!(HeaderName::from_static("X-Extend-Header").unwrap().as_bytes(), b"X-Extend-Header");
    /// ```
    pub fn as_bytes(&self) -> Vec<u8> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use rstest::rstest;

    #[test]
    fn test_from_static_pass_alpha() -> Result<()> {
        let name = HeaderName::from_static("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")?;
        assert_eq!(
            name.to_string(),
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        );

        Ok(())
    }

    #[test]
    fn test_from_static_pass_digit() -> Result<()> {
        let name = HeaderName::from_static("1234567890")?;
        assert_eq!(name.to_string(), "1234567890");

        Ok(())
    }

    #[test]
    fn test_from_static_pass_acceptable_symbol() -> Result<()> {
        let name = HeaderName::from_static("!#$%&'*+-.^_`|~")?;
        assert_eq!(name.to_string(), "!#$%&'*+-.^_`|~");

        Ok(())
    }

    #[rstest]
    #[case::nul("\0")]
    #[case::soh("\x01")]
    #[case::stx("\x02")]
    #[case::etx("\x03")]
    #[case::eot("\x04")]
    #[case::enq("\x05")]
    #[case::ack("\x06")]
    #[case::bel("\x07")]
    #[case::bs("\x08")]
    #[case::ht("\x09")]
    #[case::lf("\n")]
    #[case::vt("\x0b")]
    #[case::ff("\x0c")]
    #[case::cr("\r")]
    #[case::so("\x0e")]
    #[case::si("\x0f")]
    #[case::dle("\x10")]
    #[case::dc1("\x11")]
    #[case::dc2("\x12")]
    #[case::dc3("\x13")]
    #[case::dc4("\x14")]
    #[case::nak("\x15")]
    #[case::syn("\x16")]
    #[case::etb("\x17")]
    #[case::can("\x18")]
    #[case::em("\x19")]
    #[case::sub("\x1a")]
    #[case::esc("\x1b")]
    #[case::fs("\x1c")]
    #[case::gs("\x1d")]
    #[case::rs("\x1e")]
    #[case::us("\x1f")]
    #[case::delete("\x7f")]
    fn test_from_static_failed_control_character(#[case] input: String) -> Result<()> {
        let res = HeaderName::from_static(&input);
        assert!(res.is_err());
        matches!(res, Err(Error::InvalidHeaderName(s)) if s == input);

        Ok(())
    }

    #[rstest]
    #[case::left_parenthesis("(")]
    #[case::right_parenthesis(")")]
    #[case::comma(",")]
    #[case::slash("/")]
    #[case::semicolon(";")]
    #[case::less_than_sign("<")]
    #[case::equals_sign("=")]
    #[case::greater_than_sign(">")]
    #[case::question_mark("?")]
    #[case::at_sign("@")]
    #[case::left_square_bracket("[")]
    #[case::backslash("\\")]
    #[case::right_square_bracket("]")]
    #[case::left_curly_brace("{")]
    #[case::right_curly_brace("}")]
    fn test_from_static_failed_unavailable_symbols(#[case] input: String) -> Result<()> {
        let res = HeaderName::from_static(&input);
        assert!(res.is_err());
        matches!(res, Err(Error::InvalidHeaderName(s)) if s == input);

        Ok(())
    }
}
