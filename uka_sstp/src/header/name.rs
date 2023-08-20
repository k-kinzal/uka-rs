use std::fmt::Display;
use uka_util::string::{Error as Rfc7230StringConvertError, Rfc7230String};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Inner {
    Charset,
    Sender,
    Event,
    Reference0,
    Reference1,
    Reference2,
    Reference3,
    Reference4,
    Reference5,
    Reference6,
    Reference7,
    Script,
    Option,
    Entry,
    HWnd,
    IfGhost,
    Command,
    Document,
    Songname,
    Sentence,
    Port,
    Surface,
    Other(Rfc7230String),
}

/// Error that can occur when convert from string.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid header name: {0}")]
    InvalidHeaderName(#[from] Rfc7230StringConvertError),
}

/// HeaderName is the name of the SSTP header field.
///
/// It has some field names defined based on SSTP specifications and extended proprietary field names.
/// HeaderName is used as a key in the HeaderMap; constants are available for header names based on the SSTP specification.
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct HeaderName(Inner);

impl HeaderName {
    /// Charset
    pub const CHARSET: HeaderName = HeaderName(Inner::Charset);

    /// Sender
    pub const SENDER: HeaderName = HeaderName(Inner::Sender);

    /// Event
    pub const EVENT: HeaderName = HeaderName(Inner::Event);

    /// Reference0
    pub const REFERENCE0: HeaderName = HeaderName(Inner::Reference0);

    /// Reference1
    pub const REFERENCE1: HeaderName = HeaderName(Inner::Reference1);

    /// Reference2
    pub const REFERENCE2: HeaderName = HeaderName(Inner::Reference2);

    /// Reference3
    pub const REFERENCE3: HeaderName = HeaderName(Inner::Reference3);

    /// Reference4
    pub const REFERENCE4: HeaderName = HeaderName(Inner::Reference4);

    /// Reference5
    pub const REFERENCE5: HeaderName = HeaderName(Inner::Reference5);

    /// Reference6
    pub const REFERENCE6: HeaderName = HeaderName(Inner::Reference6);

    /// Reference7
    pub const REFERENCE7: HeaderName = HeaderName(Inner::Reference7);

    /// Script
    pub const SCRIPT: HeaderName = HeaderName(Inner::Script);

    /// Option
    pub const OPTION: HeaderName = HeaderName(Inner::Option);

    /// Entry
    pub const ENTRY: HeaderName = HeaderName(Inner::Entry);

    /// HWnd
    pub const HWND: HeaderName = HeaderName(Inner::HWnd);

    /// IfGhost
    pub const IF_GHOST: HeaderName = HeaderName(Inner::IfGhost);

    /// Command
    pub const COMMAND: HeaderName = HeaderName(Inner::Command);

    /// Document
    pub const DOCUMENT: HeaderName = HeaderName(Inner::Document);

    /// Songname
    pub const SONGNAME: HeaderName = HeaderName(Inner::Songname);

    /// Sentence
    pub const SENTENCE: HeaderName = HeaderName(Inner::Sentence);

    /// Port
    pub const PORT: HeaderName = HeaderName(Inner::Port);

    /// Surface
    pub const SURFACE: HeaderName = HeaderName(Inner::Surface);

    ///　Converts a str to HeaderName.
    ///
    /// ```rust
    /// # use uka_sstp::HeaderName;
    /// assert_eq!(HeaderName::from_static("Charset").unwrap(), HeaderName::CHARSET);
    /// assert_eq!(HeaderName::from_static("Sender").unwrap(), HeaderName::SENDER);
    /// assert_eq!(HeaderName::from_static("Event").unwrap(), HeaderName::EVENT);
    /// assert_eq!(HeaderName::from_static("Reference0").unwrap(), HeaderName::REFERENCE0);
    /// assert_eq!(HeaderName::from_static("Reference1").unwrap(), HeaderName::REFERENCE1);
    /// assert_eq!(HeaderName::from_static("Reference2").unwrap(), HeaderName::REFERENCE2);
    /// assert_eq!(HeaderName::from_static("Reference3").unwrap(), HeaderName::REFERENCE3);
    /// assert_eq!(HeaderName::from_static("Reference4").unwrap(), HeaderName::REFERENCE4);
    /// assert_eq!(HeaderName::from_static("Reference5").unwrap(), HeaderName::REFERENCE5);
    /// assert_eq!(HeaderName::from_static("Reference6").unwrap(), HeaderName::REFERENCE6);
    /// assert_eq!(HeaderName::from_static("Reference7").unwrap(), HeaderName::REFERENCE7);
    /// assert_eq!(HeaderName::from_static("Script").unwrap(), HeaderName::SCRIPT);
    /// assert_eq!(HeaderName::from_static("Option").unwrap(), HeaderName::OPTION);
    /// assert_eq!(HeaderName::from_static("Entry").unwrap(), HeaderName::ENTRY);
    /// assert_eq!(HeaderName::from_static("HWnd").unwrap(), HeaderName::HWND);
    /// assert_eq!(HeaderName::from_static("IfGhost").unwrap(), HeaderName::IF_GHOST);
    /// assert_eq!(HeaderName::from_static("Command").unwrap(), HeaderName::COMMAND);
    /// assert_eq!(HeaderName::from_static("Document").unwrap(), HeaderName::DOCUMENT);
    /// assert_eq!(HeaderName::from_static("Songname").unwrap(), HeaderName::SONGNAME);
    /// assert_eq!(HeaderName::from_static("Sentence").unwrap(), HeaderName::SENTENCE);
    /// assert_eq!(HeaderName::from_static("Port").unwrap(), HeaderName::PORT);
    /// assert_eq!(HeaderName::from_static("Surface").unwrap(), HeaderName::SURFACE);
    /// assert_eq!(HeaderName::from_static("X-Extend-Header").unwrap().to_string(), "X-Extend-Header");
    /// ```
    pub fn from_static(s: &str) -> Result<HeaderName, Error> {
        match s {
            "Charset" => Ok(HeaderName(Inner::Charset)),
            "Sender" => Ok(HeaderName(Inner::Sender)),
            "Event" => Ok(HeaderName(Inner::Event)),
            "Reference0" => Ok(HeaderName(Inner::Reference0)),
            "Reference1" => Ok(HeaderName(Inner::Reference1)),
            "Reference2" => Ok(HeaderName(Inner::Reference2)),
            "Reference3" => Ok(HeaderName(Inner::Reference3)),
            "Reference4" => Ok(HeaderName(Inner::Reference4)),
            "Reference5" => Ok(HeaderName(Inner::Reference5)),
            "Reference6" => Ok(HeaderName(Inner::Reference6)),
            "Reference7" => Ok(HeaderName(Inner::Reference7)),
            "Script" => Ok(HeaderName(Inner::Script)),
            "Option" => Ok(HeaderName(Inner::Option)),
            "Entry" => Ok(HeaderName(Inner::Entry)),
            "HWnd" => Ok(HeaderName(Inner::HWnd)),
            "IfGhost" => Ok(HeaderName(Inner::IfGhost)),
            "Command" => Ok(HeaderName(Inner::Command)),
            "Document" => Ok(HeaderName(Inner::Document)),
            "Songname" => Ok(HeaderName(Inner::Songname)),
            "Sentence" => Ok(HeaderName(Inner::Sentence)),
            "Port" => Ok(HeaderName(Inner::Port)),
            "Surface" => Ok(HeaderName(Inner::Surface)),
            _ => Ok(HeaderName(Inner::Other(Rfc7230String::from_string(
                s.to_string(),
            )?))),
        }
    }

    ///　Converts a bytes to HeaderName.
    ///
    /// ```rust
    /// # use uka_sstp::HeaderName;
    /// assert_eq!(HeaderName::from_bytes(b"Charset").unwrap(), HeaderName::CHARSET);
    /// assert_eq!(HeaderName::from_bytes(b"Sender").unwrap(), HeaderName::SENDER);
    /// assert_eq!(HeaderName::from_bytes(b"Event").unwrap(), HeaderName::EVENT);
    /// assert_eq!(HeaderName::from_bytes(b"Reference0").unwrap(), HeaderName::REFERENCE0);
    /// assert_eq!(HeaderName::from_bytes(b"Reference1").unwrap(), HeaderName::REFERENCE1);
    /// assert_eq!(HeaderName::from_bytes(b"Reference2").unwrap(), HeaderName::REFERENCE2);
    /// assert_eq!(HeaderName::from_bytes(b"Reference3").unwrap(), HeaderName::REFERENCE3);
    /// assert_eq!(HeaderName::from_bytes(b"Reference4").unwrap(), HeaderName::REFERENCE4);
    /// assert_eq!(HeaderName::from_bytes(b"Reference5").unwrap(), HeaderName::REFERENCE5);
    /// assert_eq!(HeaderName::from_bytes(b"Reference6").unwrap(), HeaderName::REFERENCE6);
    /// assert_eq!(HeaderName::from_bytes(b"Reference7").unwrap(), HeaderName::REFERENCE7);
    /// assert_eq!(HeaderName::from_bytes(b"Script").unwrap(), HeaderName::SCRIPT);
    /// assert_eq!(HeaderName::from_bytes(b"Option").unwrap(), HeaderName::OPTION);
    /// assert_eq!(HeaderName::from_bytes(b"Entry").unwrap(), HeaderName::ENTRY);
    /// assert_eq!(HeaderName::from_bytes(b"HWnd").unwrap(), HeaderName::HWND);
    /// assert_eq!(HeaderName::from_bytes(b"IfGhost").unwrap(), HeaderName::IF_GHOST);
    /// assert_eq!(HeaderName::from_bytes(b"Command").unwrap(), HeaderName::COMMAND);
    /// assert_eq!(HeaderName::from_bytes(b"Document").unwrap(), HeaderName::DOCUMENT);
    /// assert_eq!(HeaderName::from_bytes(b"Songname").unwrap(), HeaderName::SONGNAME);
    /// assert_eq!(HeaderName::from_bytes(b"Sentence").unwrap(), HeaderName::SENTENCE);
    /// assert_eq!(HeaderName::from_bytes(b"Port").unwrap(), HeaderName::PORT);
    /// assert_eq!(HeaderName::from_bytes(b"Surface").unwrap(), HeaderName::SURFACE);
    /// assert_eq!(HeaderName::from_bytes(b"X-Extend-Header").unwrap().to_string(), "X-Extend-Header");
    /// ```
    pub fn from_bytes(bytes: &[u8]) -> Result<HeaderName, Error> {
        let s = String::from_utf8_lossy(bytes);
        HeaderName::from_static(s.as_ref())
    }

    ///　Converts a HeaderName to bytes.
    ///
    /// ```rust
    /// # use uka_sstp::HeaderName;
    /// assert_eq!(HeaderName::CHARSET.to_vec(), b"Charset");
    /// assert_eq!(HeaderName::SENDER.to_vec(), b"Sender");
    /// assert_eq!(HeaderName::EVENT.to_vec(), b"Event");
    /// assert_eq!(HeaderName::REFERENCE0.to_vec(), b"Reference0");
    /// assert_eq!(HeaderName::REFERENCE1.to_vec(), b"Reference1");
    /// assert_eq!(HeaderName::REFERENCE2.to_vec(), b"Reference2");
    /// assert_eq!(HeaderName::REFERENCE3.to_vec(), b"Reference3");
    /// assert_eq!(HeaderName::REFERENCE4.to_vec(), b"Reference4");
    /// assert_eq!(HeaderName::REFERENCE5.to_vec(), b"Reference5");
    /// assert_eq!(HeaderName::REFERENCE6.to_vec(), b"Reference6");
    /// assert_eq!(HeaderName::REFERENCE7.to_vec(), b"Reference7");
    /// assert_eq!(HeaderName::SCRIPT.to_vec(), b"Script");
    /// assert_eq!(HeaderName::OPTION.to_vec(), b"Option");
    /// assert_eq!(HeaderName::ENTRY.to_vec(), b"Entry");
    /// assert_eq!(HeaderName::HWND.to_vec(), b"HWnd");
    /// assert_eq!(HeaderName::IF_GHOST.to_vec(), b"IfGhost");
    /// assert_eq!(HeaderName::COMMAND.to_vec(), b"Command");
    /// assert_eq!(HeaderName::DOCUMENT.to_vec(), b"Document");
    /// assert_eq!(HeaderName::SONGNAME.to_vec(), b"Songname");
    /// assert_eq!(HeaderName::SENTENCE.to_vec(), b"Sentence");
    /// assert_eq!(HeaderName::PORT.to_vec(), b"Port");
    /// assert_eq!(HeaderName::SURFACE.to_vec(), b"Surface");
    /// assert_eq!(HeaderName::from_static("X-Extend-Header").unwrap().to_vec(), b"X-Extend-Header");
    /// ```
    pub fn to_vec(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}

impl Display for HeaderName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Inner::Charset => write!(f, "Charset"),
            Inner::Sender => write!(f, "Sender"),
            Inner::Event => write!(f, "Event"),
            Inner::Reference0 => write!(f, "Reference0"),
            Inner::Reference1 => write!(f, "Reference1"),
            Inner::Reference2 => write!(f, "Reference2"),
            Inner::Reference3 => write!(f, "Reference3"),
            Inner::Reference4 => write!(f, "Reference4"),
            Inner::Reference5 => write!(f, "Reference5"),
            Inner::Reference6 => write!(f, "Reference6"),
            Inner::Reference7 => write!(f, "Reference7"),
            Inner::Script => write!(f, "Script"),
            Inner::Option => write!(f, "Option"),
            Inner::Entry => write!(f, "Entry"),
            Inner::HWnd => write!(f, "HWnd"),
            Inner::IfGhost => write!(f, "IfGhost"),
            Inner::Command => write!(f, "Command"),
            Inner::Document => write!(f, "Document"),
            Inner::Songname => write!(f, "Songname"),
            Inner::Sentence => write!(f, "Sentence"),
            Inner::Port => write!(f, "Port"),
            Inner::Surface => write!(f, "Surface"),
            Inner::Other(s) => write!(f, "{s}"),
        }
    }
}

impl From<HeaderName> for String {
    fn from(header_name: HeaderName) -> Self {
        header_name.to_string()
    }
}
