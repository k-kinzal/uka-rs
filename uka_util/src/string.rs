use std::fmt::Display;
use std::ops::Deref;

/// Errors that may occur during conversion.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid character: `{0}`")]
    InvalidCharacter(u8),
}

type Result<T> = std::result::Result<T, Error>;

/// Rfc7230String is a string that conforms to the character set defined in RFC 7230.
/// Alpa-numeric characters, some symbols are allowed.
///
/// See: https://triple-underscore.github.io/RFC7230-ja.html#section-3.2.6
///
/// # Examples
///
/// ```rust
/// # use uka_util::string::Rfc7230String;
/// #
/// let name = Rfc7230String::from_string("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!#$%&'*+-.^_`|~".to_string());
/// assert!(name.is_ok());
/// assert_eq!(name.unwrap().to_string(), "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!#$%&'*+-.^_`|~");
/// ```
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Rfc7230String(String);

impl Rfc7230String {
    /// Create a new Rfc7230String.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uka_util::string::Rfc7230String;
    /// #
    /// let name = Rfc7230String::new();
    /// assert_eq!(name.to_string(), "");
    /// ```
    pub fn new() -> Self {
        Self(String::new())
    }

    /// Create a new Rfc7230String from a bytes.
    ///
    /// # Errors
    ///
    /// If the bytes contains invalid characters, an error will be returned.
    ///　Valid characters are as follows:
    /// - Lowercase letters: `abcdefghijklmnopqrstuvwxyz`
    /// - Uppercase letters: `ABCDEFGHIJKLMNOPQRSTUVWXYZ`
    /// - Numbers: `0123456789`
    /// - Symbols: `!#$%&'*+-.^_`|~`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uka_util::string::Rfc7230String;
    /// #
    /// let name = Rfc7230String::from_utf8(b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!#$%&'*+-.^_`|~".to_vec());
    ///  assert!(name.is_ok());
    /// assert_eq!(name.unwrap().to_string(), "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!#$%&'*+-.^_`|~");
    /// ```
    pub fn from_utf8(bytes: Vec<u8>) -> Result<Self> {
        let opt = bytes.iter().find(|byte| !matches!(*byte, b'0'..=b'9' | b'!' | b'#'..=b'\'' | b'*'..=b'+' | b'-'..=b'.' | b'^'..=b'`' | b'A'..=b'Z' | b'a'..=b'z' | b'|' | b'~'));
        if let Some(c) = opt {
            Err(Error::InvalidCharacter(c.clone()))
        } else {
            Ok(Self(
                String::from_utf8(bytes).expect("unreachable: within the range of UTF-8"),
            ))
        }
    }

    /// Create a new Rfc7230String from a string.
    /// This function is equivalent to `Rfc7230String::from_utf8(s.into_bytes())`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uka_util::string::Rfc7230String;
    /// #
    /// let name = Rfc7230String::from_string("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!#$%&'*+-.^_`|~".to_string());
    /// assert!(name.is_ok());
    /// assert_eq!(name.unwrap().to_string(), "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!#$%&'*+-.^_`|~");
    /// ```
    pub fn from_string(s: String) -> Result<Self> {
        Self::from_utf8(s.into_bytes())
    }
}

impl Deref for Rfc7230String {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Rfc7230String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use rstest::rstest;

    #[test]
    fn test_from_utf8_pass_alpha() -> Result<()> {
        let name = Rfc7230String::from_utf8(
            b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".to_vec(),
        )?;
        assert_eq!(
            name.as_str(),
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        );

        Ok(())
    }

    #[test]
    fn test_from_utf8_pass_digit() -> Result<()> {
        let name = Rfc7230String::from_utf8(b"1234567890".to_vec())?;
        assert_eq!(name.as_str(), "1234567890");

        Ok(())
    }

    #[test]
    fn test_from_utf8_pass_acceptable_symbol() -> Result<()> {
        let name = Rfc7230String::from_utf8(b"!#$%&'*+-.^_`|~".to_vec())?;
        assert_eq!(name.as_str(), "!#$%&'*+-.^_`|~");

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
    fn test_from_utf8_failed_control_character(#[case] input: String) -> Result<()> {
        let res = Rfc7230String::from_utf8(input.into_bytes());
        assert!(res.is_err());
        matches!(res, Err(Error::InvalidCharacter(_)));

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
    fn test_from_utf8_failed_unavailable_symbols(#[case] input: String) -> Result<()> {
        let res = Rfc7230String::from_utf8(input.into_bytes());
        assert!(res.is_err());
        matches!(res, Err(Error::InvalidCharacter(_)));

        Ok(())
    }

    #[test]
    fn test_from_utf8_failed_out_of_ascii_range() -> Result<()> {
        let res = Rfc7230String::from_utf8(b"\x80".to_vec());
        assert!(res.is_err());
        matches!(res, Err(Error::InvalidCharacter(_)));

        Ok(())
    }
}
