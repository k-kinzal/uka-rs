use std::fmt;

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
enum Inner {
    Notify,
    Send,
    Execute,
    Give,
    Communicate,
}

/// Method is the method of the SSTP request.
///
/// ```rust
/// # use sstp::Method;
/// # let method = Method::NOTIFY;
/// match method {
///     Method::NOTIFY => assert_eq!(method.to_string(), "NOTIFY"),
///     Method::SEND => assert_eq!(method.to_string(), "SEND"),
///     Method::EXECUTE => assert_eq!(method.to_string(), "EXECUTE"),
///     Method::GIVE => assert_eq!(method.to_string(), "GIVE"),
///     Method::COMMUNICATE => assert_eq!(method.to_string(), "COMMUNICATE"),
/// }
/// ```
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
pub struct Method(Inner);
impl Method {
    /// Notify
    pub const NOTIFY: Method = Method(Inner::Notify);

    /// Send
    pub const SEND: Method = Method(Inner::Send);

    /// Execute
    pub const EXECUTE: Method = Method(Inner::Execute);

    /// Give
    pub const GIVE: Method = Method(Inner::Give);

    /// Communicate
    pub const COMMUNICATE: Method = Method(Inner::Communicate);
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Inner::*;

        f.write_str(match self.0 {
            Notify => "NOTIFY",
            Send => "SEND",
            Execute => "EXECUTE",
            Give => "GIVE",
            Communicate => "COMMUNICATE",
        })
    }
}
