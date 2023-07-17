use std::fmt;

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
enum Inner {
    Get,
    Notify,
}

/// Method is the method of the SHIORI request.
///
/// ```rust
/// # use uka_shiori::types::v3::Method;
/// # let method = Method::GET;
/// match method {
///     Method::GET => assert_eq!(method.to_string(), "GET"),
///     Method::NOTIFY => assert_eq!(method.to_string(), "NOTIFY"),
/// }
/// ```
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
pub struct Method(Inner);
impl Method {
    /// Get
    pub const GET: Method = Method(Inner::Get);

    /// Notify
    pub const NOTIFY: Method = Method(Inner::Notify);
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Inner::*;

        f.write_str(match self.0 {
            Get => "GET",
            Notify => "NOTIFY",
        })
    }
}
