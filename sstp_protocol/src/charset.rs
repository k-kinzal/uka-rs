use std::fmt;

/// Error that can occur when converting Charset from string.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(
        "invalid charset `{0}`, the charset must be ASCII, Shift_JIS, ISO-2022-JP, EUC-JP or UTF-8"
    )]
    Invalid(String),
}
type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
enum Inner {
    Ascii,
    ShiftJis,
    Iso2022Jp,
    EucJp,
    Utf8,
}

/// Charset is the character set and encoding of strings in SSTP headers.
///
/// ```rust
/// # use sstp_protocol::Charset;
/// #
/// # let charset = Charset::ASCII;
/// match charset {
///     Charset::ASCII => assert_eq!(charset.to_string(), "ASCII"),
///     Charset::SHIFT_JIS => assert_eq!(charset.to_string(), "Shift_JIS"),
///     Charset::ISO2022JP => assert_eq!(charset.to_string(), "ISO-2022-JP"),
///     Charset::EUC_JP => assert_eq!(charset.to_string(), "EUC-JP"),
///     Charset::UTF8 => assert_eq!(charset.to_string(), "UTF-8"),
/// }
/// ```
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
pub struct Charset(Inner);
impl Charset {
    /// ASCII
    pub const ASCII: Charset = Charset(Inner::Ascii);

    /// Shift_JIS
    pub const SHIFT_JIS: Charset = Charset(Inner::ShiftJis);

    /// ISO-2022-JP
    pub const ISO2022JP: Charset = Charset(Inner::Iso2022Jp);

    /// EUC-JP
    pub const EUC_JP: Charset = Charset(Inner::EucJp);

    /// UTF-8
    pub const UTF8: Charset = Charset(Inner::Utf8);

    ///　Converts a str to Charset.
    ///
    /// ```rust
    /// # use sstp_protocol::Charset;
    ///
    /// assert_eq!(Charset::ASCII, Charset::from_static("ASCII").unwrap());
    /// assert_eq!(Charset::SHIFT_JIS, Charset::from_static("Shift_JIS").unwrap());
    /// assert_eq!(Charset::ISO2022JP, Charset::from_static("ISO-2022-JP").unwrap());
    /// assert_eq!(Charset::EUC_JP, Charset::from_static("EUC-JP").unwrap());
    /// assert_eq!(Charset::UTF8, Charset::from_static("UTF-8").unwrap());
    /// ```
    pub fn from_static(s: &str) -> Result<Charset> {
        match s {
            "ASCII" => Ok(Charset::ASCII),
            "Shift_JIS" => Ok(Charset::SHIFT_JIS),
            "ISO-2022-JP" => Ok(Charset::ISO2022JP),
            "EUC-JP" => Ok(Charset::EUC_JP),
            "UTF-8" => Ok(Charset::UTF8),
            _ => Err(Error::Invalid(s.to_string())),
        }
    }

    ///　Converts a string to Charset.
    pub fn from_string(s: impl Into<String>) -> Result<Charset> {
        Charset::from_static(s.into().as_str())
    }
}

impl fmt::Display for Charset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Inner::*;

        f.write_str(match self.0 {
            Ascii => "ASCII",
            ShiftJis => "Shift_JIS",
            Iso2022Jp => "ISO-2022-JP",
            EucJp => "EUC-JP",
            Utf8 => "UTF-8",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_static_invalid_charset() {
        let result = Charset::from_static("Unsupported Charset");
        assert!(result.is_err());
        matches!(result.unwrap_err(), Error::Invalid(_));
    }

    #[test]
    fn test_from_string_invalid_charset() {
        let result = Charset::from_string("Unsupported Charset");
        assert!(result.is_err());
        matches!(result.unwrap_err(), Error::Invalid(_));
    }
}
