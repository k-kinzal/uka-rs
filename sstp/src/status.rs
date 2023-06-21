use std::fmt;
use std::fmt::Display;

/// StatusCode represents the status code of the SSTP response.
///
/// ```rust
/// # use sstp::StatusCode;
/// assert_eq!(StatusCode::OK.to_string(), "200 OK");
/// assert_eq!(StatusCode::NO_CONTENT.to_string(), "204 No Content");
/// assert_eq!(StatusCode::BREAK.to_string(), "210 Break");
/// assert_eq!(StatusCode::BAD_REQUEST.to_string(), "400 Bad Request");
/// assert_eq!(StatusCode::REQUEST_TIMEOUT.to_string(), "408 Request Timeout");
/// assert_eq!(StatusCode::CONFLICT.to_string(), "409 Conflict");
/// assert_eq!(StatusCode::REFUSE.to_string(), "420 Refuse");
/// assert_eq!(StatusCode::NOT_IMPLEMENTED.to_string(), "501 Not Implemented");
/// assert_eq!(StatusCode::SERVICE_UNAVAILABLE.to_string(), "503 Service Unavailable");
/// assert_eq!(StatusCode::NOT_LOCAL_IP.to_string(), "510 Not Local IP");
/// assert_eq!(StatusCode::IN_BLACK_LIST.to_string(), "511 In Black List");
/// assert_eq!(StatusCode::INVISIBLE.to_string(), "512 Invisible");
/// ```
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
pub struct StatusCode(u16);
impl StatusCode {
    // 2xx - Process Completed
    /// 200 OK
    pub const OK: StatusCode = StatusCode(200);

    /// 204 No Content
    pub const NO_CONTENT: StatusCode = StatusCode(204);

    /// 210 Break
    pub const BREAK: StatusCode = StatusCode(210);

    // 4xx - Request Error
    /// 400 Bad Request
    pub const BAD_REQUEST: StatusCode = StatusCode(400);

    /// 408 Request Timeout
    pub const REQUEST_TIMEOUT: StatusCode = StatusCode(408);

    /// 409 Conflict
    pub const CONFLICT: StatusCode = StatusCode(409);

    /// 420 Refuse
    pub const REFUSE: StatusCode = StatusCode(420);

    // 5xx - Server Error
    /// 501 Not Implemented
    pub const NOT_IMPLEMENTED: StatusCode = StatusCode(501);

    /// 503 Service Unavailable
    pub const SERVICE_UNAVAILABLE: StatusCode = StatusCode(503);

    /// 510 Not Local IP
    pub const NOT_LOCAL_IP: StatusCode = StatusCode(510);

    /// 511 In Black List
    pub const IN_BLACK_LIST: StatusCode = StatusCode(511);

    /// 512 Invisible
    pub const INVISIBLE: StatusCode = StatusCode(512);
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            200 => write!(f, "200 OK"),
            204 => write!(f, "204 No Content"),
            210 => write!(f, "210 Break"),
            400 => write!(f, "400 Bad Request"),
            408 => write!(f, "408 Request Timeout"),
            409 => write!(f, "409 Conflict"),
            420 => write!(f, "420 Refuse"),
            501 => write!(f, "501 Not Implemented"),
            503 => write!(f, "503 Service Unavailable"),
            510 => write!(f, "510 Not Local IP"),
            511 => write!(f, "511 In Black List"),
            512 => write!(f, "512 Invisible"),
            _ => unreachable!("unreachable because StatusCode cannot be new instance"),
        }
    }
}
