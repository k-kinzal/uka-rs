use std::ffi::OsString;
use std::fmt;
use std::str::FromStr;

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

impl FromStr for Version {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SHIORI/3.0" => Ok(Version::SHIORI_30),
            _ => Err(()),
        }
    }
}

impl From<String> for Version {
    fn from(s: String) -> Self {
        Version::from_str(&s).expect("unreachable: failed to parse version")
    }
}

impl From<OsString> for Version {
    fn from(s: OsString) -> Self {
        Version::from_str(&s.to_string_lossy()).expect("unreachable: failed to parse version")
    }
}
