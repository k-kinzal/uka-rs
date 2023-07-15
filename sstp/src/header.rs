pub use name::{Error as HeaderNameError, HeaderName};
use uka_util::bag::OrderedBag;
pub use value::{Error as HeaderValueError, HeaderValue};

mod name;
mod value;

pub type HeaderMap = OrderedBag<HeaderName, HeaderValue>;
