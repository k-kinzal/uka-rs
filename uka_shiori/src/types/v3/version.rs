use std::fmt;

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
enum Protocol {
    SHIORI30,
}

/// Version is the version of the SHIORI protocol.
///
/// # Examples
///
/// ```rust
/// # use uka_shiori::types::v3::Version;
/// # let version = Version::SHIORI_30;
/// match version {
///     Version::SHIORI_30 => assert_eq!(version.to_string(), "SHIORI/3.0"),
/// }
/// ```
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
pub struct Version(Protocol);
impl Version {
    /// SHIORI/3.0
    pub const SHIORI_30: Version = Version(Protocol::SHIORI30);
}

impl Default for Version {
    #[inline]
    fn default() -> Version {
        // Default to the latest version
        Version::SHIORI_30
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Protocol::*;

        f.write_str(match self.0 {
            SHIORI30 => "SHIORI/3.0",
        })
    }
}
