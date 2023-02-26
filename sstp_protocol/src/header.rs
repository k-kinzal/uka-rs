use std::collections::HashMap;

use crate::decode::Decoder;
use crate::encode::Encoder;
use crate::{decode, encode, Charset};
use std::fmt::Display;
use std::string::ToString;

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
    Other(String),
}

/// Error that can occur when convert from string.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid header name: {0}")]
    InvalidHeaderName(String),
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
    /// # use sstp_protocol::HeaderName;
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
            _ => {
                let valid = s.as_bytes().iter().all(|byte| matches!(*byte, b'0'..=b'9' | b'!' | b'#'..=b'\'' | b'*'..=b'+' | b'-'..=b'.' | b'^'..=b'`' | b'A'..=b'Z' | b'a'..=b'z' | b'|' | b'~'));
                if valid {
                    Ok(HeaderName(Inner::Other(s.to_string())))
                } else {
                    Err(Error::InvalidHeaderName(s.to_string()))
                }
            }
        }
    }

    ///　Converts a bytes to HeaderName.
    ///
    /// ```rust
    /// # use sstp_protocol::HeaderName;
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
    /// # use sstp_protocol::HeaderName;
    /// assert_eq!(HeaderName::CHARSET.as_bytes(), b"Charset");
    /// assert_eq!(HeaderName::SENDER.as_bytes(), b"Sender");
    /// assert_eq!(HeaderName::EVENT.as_bytes(), b"Event");
    /// assert_eq!(HeaderName::REFERENCE0.as_bytes(), b"Reference0");
    /// assert_eq!(HeaderName::REFERENCE1.as_bytes(), b"Reference1");
    /// assert_eq!(HeaderName::REFERENCE2.as_bytes(), b"Reference2");
    /// assert_eq!(HeaderName::REFERENCE3.as_bytes(), b"Reference3");
    /// assert_eq!(HeaderName::REFERENCE4.as_bytes(), b"Reference4");
    /// assert_eq!(HeaderName::REFERENCE5.as_bytes(), b"Reference5");
    /// assert_eq!(HeaderName::REFERENCE6.as_bytes(), b"Reference6");
    /// assert_eq!(HeaderName::REFERENCE7.as_bytes(), b"Reference7");
    /// assert_eq!(HeaderName::SCRIPT.as_bytes(), b"Script");
    /// assert_eq!(HeaderName::OPTION.as_bytes(), b"Option");
    /// assert_eq!(HeaderName::ENTRY.as_bytes(), b"Entry");
    /// assert_eq!(HeaderName::HWND.as_bytes(), b"HWnd");
    /// assert_eq!(HeaderName::IF_GHOST.as_bytes(), b"IfGhost");
    /// assert_eq!(HeaderName::COMMAND.as_bytes(), b"Command");
    /// assert_eq!(HeaderName::DOCUMENT.as_bytes(), b"Document");
    /// assert_eq!(HeaderName::SONGNAME.as_bytes(), b"Songname");
    /// assert_eq!(HeaderName::SENTENCE.as_bytes(), b"Sentence");
    /// assert_eq!(HeaderName::PORT.as_bytes(), b"Port");
    /// assert_eq!(HeaderName::SURFACE.as_bytes(), b"Surface");
    /// assert_eq!(HeaderName::from_static("X-Extend-Header").unwrap().as_bytes(), b"X-Extend-Header");
    /// ```
    pub fn as_bytes(&self) -> Vec<u8> {
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

/// HeaderValue is the value of the SSTP header field.
///
/// The value is held in a byte string of the character set and encoding that can be specified in Charset.
/// Since it is not possible to determine which SSTP header fields are allowed to contain multibyte characters
/// and which actually contain multibyte characters,
/// users should specify them explicitly when retrieving them.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct HeaderValue(Vec<u8>);
impl HeaderValue {
    /// Extract HeaderValue as an ASCII code string.
    ///
    /// ```rust
    /// # use sstp_protocol::{HeaderValue};
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// assert_eq!(HeaderValue::from_static("sakura")?.text()?, "sakura");
    /// #     Ok(())
    /// # }
    /// ```
    pub fn text(&self) -> Result<String, decode::Error> {
        self.text_with_charset(Charset::ASCII)
    }

    /// Extract HeaderValue as a string with Charset
    /// ```rust
    /// # use sstp_protocol::{HeaderValue, Charset};
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// let input = [130, 179, 130, 173, 130, 231].to_vec();
    /// assert_eq!(HeaderValue::from(input).text_with_charset(Charset::SHIFT_JIS)?, "さくら");
    /// #     Ok(())
    /// # }
    /// ```
    pub fn text_with_charset(&self, charset: Charset) -> Result<String, decode::Error> {
        match charset {
            Charset::ASCII => Decoder::decode_ascii(&self.0),
            Charset::SHIFT_JIS => Decoder::decode_sjis(&self.0),
            Charset::ISO2022JP => Decoder::decode_iso_2022_jp(&self.0),
            Charset::EUC_JP => Decoder::decode_euc_jp(&self.0),
            Charset::UTF8 => Decoder::decode_utf8(&self.0),
        }
    }

    ///　Convert string to HeaderValue with ASCII code bytes.
    ///
    /// ```rust
    /// # use sstp_protocol::{HeaderValue};
    /// assert_eq!(HeaderValue::from_static("sakura").unwrap().as_bytes(), b"sakura");
    /// ```
    pub fn from_static(s: &str) -> Result<Self, encode::Error> {
        Self::from_static_with_charset(s, Charset::ASCII)
    }

    ///　Convert string to HeaderValue with Charset.
    ///
    ///```rust
    /// # use sstp_protocol::{Charset, HeaderValue};
    /// assert_eq!(
    ///    HeaderValue::from_static_with_charset("さくら", Charset::SHIFT_JIS).unwrap().as_bytes(),
    ///    [130, 179, 130, 173, 130, 231]);
    /// ```
    pub fn from_static_with_charset(s: &str, charset: Charset) -> Result<Self, encode::Error> {
        let bytes = match charset {
            Charset::ASCII => Encoder::encode_ascii(s)?,
            Charset::SHIFT_JIS => Encoder::encode_sjis(s)?,
            Charset::ISO2022JP => Encoder::encode_iso_2022_jp(s)?,
            Charset::EUC_JP => Encoder::encode_euc_jp(s)?,
            Charset::UTF8 => Encoder::encode_utf8(s)?,
        };
        Ok(Self(bytes))
    }

    /// Convert HeaderValue to bytes.
    pub fn as_bytes(&self) -> Vec<u8> {
        self.0.clone()
    }
}

impl From<&[u8]> for HeaderValue {
    fn from(s: &[u8]) -> Self {
        Self(s.to_vec())
    }
}

impl From<Vec<u8>> for HeaderValue {
    fn from(s: Vec<u8>) -> Self {
        Self(s)
    }
}

/// Pos represents the position in which the header appears in the SSTP header field.
type Pos = usize;

/// Entry represents the SSTP header field.
#[derive(Debug)]
struct Entry {
    name: HeaderName,
    value: HeaderValue,
}

/// HeaderMap is a bag of SSTP header fields.
///
/// SSTP header fields are allowed to have multiple identical field names.
/// The HeaderMap preserves the order of multiple identical field names
/// and can return one or all of the fields from the field name.
#[derive(Debug, Default)]
pub struct HeaderMap {
    entries: Vec<Entry>,
    map: HashMap<HeaderName, Vec<Pos>>,
}

impl HeaderMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: Vec::with_capacity(capacity),
            map: HashMap::with_capacity(capacity),
        }
    }

    /// Returns only one value for a field name.
    ///
    /// ```rust
    /// # use sstp_protocol::{HeaderMap, HeaderName, HeaderValue};
    /// let mut header_map = HeaderMap::new();
    /// header_map.insert(HeaderName::SENDER, HeaderValue::from_static("sakura").unwrap());
    /// header_map.insert(HeaderName::SENDER, HeaderValue::from_static("naru").unwrap());
    /// assert_eq!(
    ///     header_map.get(HeaderName::SENDER).and_then(|v| v.text().ok()),
    ///     Some("sakura".to_string()));
    /// ```
    pub fn get<K>(&self, key: K) -> Option<&HeaderValue>
    where
        K: Into<HeaderName>,
    {
        self.map
            .get(&key.into())
            .and_then(|v| v.first())
            .map(|i| &self.entries[*i].value)
    }

    /// Returns all values for a field name.
    ///
    /// ```rust
    /// # use sstp_protocol::{HeaderMap, HeaderName, HeaderValue};
    /// let mut header_map = HeaderMap::new();
    /// header_map.insert(HeaderName::SENDER, HeaderValue::from(b"sakura".to_vec()));
    /// header_map.insert(HeaderName::SENDER, HeaderValue::from(b"naru".to_vec()));
    /// assert_eq!(
    ///     header_map.get_all(HeaderName::SENDER).iter().map(|v| v.text().unwrap()).collect::<Vec<String>>(),
    ///     vec!["sakura".to_string(), "naru".to_string()])
    /// ```
    pub fn get_all<K>(&self, key: K) -> Vec<&HeaderValue>
    where
        K: Into<HeaderName>,
    {
        self.map
            .get(&key.into())
            .map(|v| v.iter().map(|i| &self.entries[*i].value).collect())
            .unwrap_or_default()
    }

    /// Insert field name and value
    pub fn insert<K, V>(&mut self, key: K, value: V)
    where
        K: Into<HeaderName>,
        V: Into<HeaderValue>,
    {
        let name = key.into();
        self.entries.push(Entry {
            name: name.clone(),
            value: value.into(),
        });
        self.map
            .entry(name)
            .or_default()
            .push(self.entries.len() - 1);
    }

    pub fn iter(&self) -> impl Iterator<Item = (&HeaderName, &HeaderValue)> {
        self.entries.iter().map(|e| (&e.name, &e.value))
    }
}

impl IntoIterator for HeaderMap {
    type Item = (HeaderName, HeaderValue);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries
            .into_iter()
            .map(|e| (e.name, e.value))
            .collect::<Vec<_>>()
            .into_iter()
    }
}
