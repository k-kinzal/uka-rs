use crate::header::name::HeaderName;
use crate::header::value::HeaderValue;
use std::collections::HashMap;

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
