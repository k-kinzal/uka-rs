use std::fmt;
use std::fmt::Display;

/// StatusCode represents the status code of the SHIORI response.
///
/// # Examples
///
/// ```rust
/// # use uka_shiori::types::v3::StatusCode;
/// assert_eq!(StatusCode::OK.to_string(), "200 OK");
/// assert_eq!(StatusCode::NO_CONTENT.to_string(), "204 No Content");
/// assert_eq!(StatusCode::COMMUNICATE.to_string(), "310 Communicate");
/// assert_eq!(StatusCode::NOT_ENOUGH.to_string(), "311 Not Enough");
/// assert_eq!(StatusCode::ADVICE.to_string(), "312 Advice");
/// assert_eq!(StatusCode::BAD_REQUEST.to_string(), "400 Bad Request");
/// assert_eq!(StatusCode::INTERNAL_SERVER_ERROR.to_string(), "500 Internal Server Error");
/// ```
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
pub struct StatusCode(u16);
impl StatusCode {
    // 2xx - Process Completed
    /// 200 OK
    pub const OK: StatusCode = StatusCode(200);

    /// 204 No Content
    pub const NO_CONTENT: StatusCode = StatusCode(204);

    // 3xx - 処理完了、追加アクション要求
    /// 310 Communicate (deprecated)
    pub const COMMUNICATE: StatusCode = StatusCode(310);

    /// 311 Not Enough
    pub const NOT_ENOUGH: StatusCode = StatusCode(311);

    /// 312 Advice
    pub const ADVICE: StatusCode = StatusCode(312);

    // 4xx - Request Error
    /// 400 Bad Request
    pub const BAD_REQUEST: StatusCode = StatusCode(400);

    // 5xx - Server Error
    /// 500 Internal Server Error
    pub const INTERNAL_SERVER_ERROR: StatusCode = StatusCode(500);
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            200 => write!(f, "200 OK"),
            204 => write!(f, "204 No Content"),
            310 => write!(f, "310 Communicate"),
            311 => write!(f, "311 Not Enough"),
            312 => write!(f, "312 Advice"),
            400 => write!(f, "400 Bad Request"),
            500 => write!(f, "500 Internal Server Error"),
            _ => unreachable!("unreachable because StatusCode cannot be new instance"),
        }
    }
}
