use std::fmt;

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
pub enum Version {
    SSTP_10,
    SSTP_11,
    SSTP_12,
    SSTP_13,
    SSTP_14,
}

impl Version {
    // /// SSTP/1.0
    // pub const SSTP_10: Version = Version(Protocol::Sstp10);
    //
    // /// SSTP/1.1
    // pub const SSTP_11: Version = Version(Protocol::Sstp11);
    //
    // /// SSTP/1.2
    // pub const SSTP_12: Version = Version(Protocol::Sstp12);
    //
    // /// SSTP/1.3
    // pub const SSTP_13: Version = Version(Protocol::Sstp13);
    //
    // /// SSTP/1.4
    // pub const SSTP_14: Version = Version(Protocol::Sstp14);
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
        use Version::*;

        f.write_str(match self {
            SSTP_10 => "SSTP/1.0",
            SSTP_11 => "SSTP/1.1",
            SSTP_12 => "SSTP/1.2",
            SSTP_13 => "SSTP/1.3",
            SSTP_14 => "SSTP/1.4",
        })
    }
}
