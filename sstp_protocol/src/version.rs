use std::fmt;

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
enum Protocol {
    Sstp10,
    Sstp11,
    Sstp12,
    Sstp13,
    Sstp14,
}

/// Version is the version of the SSTP protocol.
///
/// ```rust
/// # use sstp_protocol::Version;
/// # let version = Version::SSTP_14;
/// match version {
///     Version::SSTP_10 => assert_eq!(version.to_string(), "SSTP/1.0"),
///     Version::SSTP_11 => assert_eq!(version.to_string(), "SSTP/1.1"),
///     Version::SSTP_12 => assert_eq!(version.to_string(), "SSTP/1.2"),
///     Version::SSTP_13 => assert_eq!(version.to_string(), "SSTP/1.3"),
///     Version::SSTP_14 => assert_eq!(version.to_string(), "SSTP/1.4"),
/// }
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
pub struct Version(Protocol);
impl Version {
    /// SSTP/1.0
    pub const SSTP_10: Version = Version(Protocol::Sstp10);

    /// SSTP/1.1
    pub const SSTP_11: Version = Version(Protocol::Sstp11);

    /// SSTP/1.2
    pub const SSTP_12: Version = Version(Protocol::Sstp12);

    /// SSTP/1.3
    pub const SSTP_13: Version = Version(Protocol::Sstp13);

    /// SSTP/1.4
    pub const SSTP_14: Version = Version(Protocol::Sstp14);
}
impl Default for Version {
    #[inline]
    fn default() -> Version {
        // Default to the latest version
        Version::SSTP_14
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Protocol::*;

        f.write_str(match self.0 {
            Sstp10 => "SSTP/1.0",
            Sstp11 => "SSTP/1.1",
            Sstp12 => "SSTP/1.2",
            Sstp13 => "SSTP/1.3",
            Sstp14 => "SSTP/1.4",
        })
    }
}
