use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

/// A type alias for a position in an array or other data structure.
/// This is typically used to index into an array or vector.
type Pos = usize;

/// An ordered key-value bag.
///
/// This struct is similar to `HashMap` in that it stores key-value pairs,
/// but unlike `HashMap`, it also maintains the order of insertion.
/// As such, you can iterate over the entries in an `OrderedBag`
/// in the order they were inserted.
///
/// # Examples
///
/// ```rust
/// # use uka_util::bag::OrderedBag;
/// #
/// let mut bag = OrderedBag::new();
/// bag.insert("key1", "value1");
/// bag.insert("key2", "value2");
/// assert_eq!(bag.get("key1"), Some(&"value1"));
/// ```
#[derive(Debug, Default)]
pub struct OrderedBag<K, V> {
    entries: Vec<(K, V)>,
    map: HashMap<K, Vec<Pos>>,
}

impl<K: Eq + Hash, V> OrderedBag<K, V> {
    /// Constructs a new, empty `OrderedBag<K, V>`.
    ///
    /// The `OrderedBag` is initially created with a capacity of 0, so it will not allocate until it is first inserted into.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            map: HashMap::new(),
        }
    }

    /// Constructs a new, empty `OrderedBag<K, V>` with the specified capacity.
    ///
    /// The `OrderedBag` will be able to hold exactly `capacity` elements without reallocating.
    /// If `capacity` is 0, the bag will not allocate.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: Vec::with_capacity(capacity),
            map: HashMap::with_capacity(capacity),
        }
    }

    /// Returns a reference to the value corresponding to the key.
    /// If more than one value is inserted for a key, get always returns the first value inserted.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uka_util::bag::OrderedBag;
    /// #
    /// let mut bag = OrderedBag::new();
    /// bag.insert("key1", "value1");
    /// assert_eq!(bag.get("key1"), Some(&"value1"));
    /// ```
    pub fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        Q: ?Sized + Eq + Hash,
        K: Borrow<Q>,
    {
        self.map
            .get(k)
            .and_then(|pos| pos.first())
            .map(|pos| &self.entries[*pos].1)
    }

    /// Returns a `Vec` of references to all values corresponding to the key.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uka_util::bag::OrderedBag;
    /// #
    /// let mut bag = OrderedBag::new();
    /// bag.insert("key1", "value1");
    /// bag.insert("key1", "value2");
    ///
    /// assert_eq!(bag.get_all("key1"), [&"value1", &"value2"].to_vec());
    /// ```
    pub fn get_all<Q>(&self, k: &Q) -> Vec<&V>
    where
        Q: ?Sized + Eq + Hash,
        K: Borrow<Q>,
    {
        self.map
            .get(k)
            .map(|pos| pos.iter().map(|pos| &self.entries[*pos].1).collect())
            .unwrap_or_default()
    }

    /// Inserts a key-value pair into the bag.
    /// If the key already exists, add a value to it
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uka_util::bag::OrderedBag;
    /// #
    /// let mut bag = OrderedBag::new();
    /// bag.insert("key1", "value1");
    /// assert_eq!(bag.get("key1"), Some(&"value1"));
    /// ```
    pub fn insert(&mut self, k: K, v: V)
    where
        K: Clone, // TODO: Remove the clone
    {
        let pos = self.entries.len();
        self.entries.push((k.clone(), v));
        self.map.entry(k).or_default().push(pos);
    }

    /// Returns the number of elements in the bag.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns true if the bag contains no elements.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns an iterator over the bag.
    /// The iterator will yield tuples in the order they were inserted into the bag.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uka_util::bag::OrderedBag;
    /// #
    /// let mut bag = OrderedBag::new();
    /// bag.insert("key1", "value1");
    /// bag.insert("key2", "value2");
    ///
    /// let mut iter = bag.iter();
    /// assert_eq!(iter.next(), Some(&("key1", "value1")));
    /// assert_eq!(iter.next(), Some(&("key2", "value2")));
    /// assert!(iter.next().is_none());
    /// ```
    pub fn iter(&self) -> std::slice::Iter<'_, (K, V)> {
        self.entries.iter()
    }
}

impl<K, V> IntoIterator for OrderedBag<K, V> {
    type Item = (K, V);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

impl<K: Eq + Hash + Clone, V> From<Vec<(K, V)>> for OrderedBag<K, V> {
    fn from(entries: Vec<(K, V)>) -> Self {
        let mut bag = Self::with_capacity(entries.len());
        for (k, v) in entries {
            bag.insert(k, v);
        }
        bag
    }
}

impl<K: Eq + Hash + Clone, V> FromIterator<(K, V)> for OrderedBag<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut bag = Self::new();
        for (k, v) in iter {
            bag.insert(k, v);
        }
        bag
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordered_bag_get() {
        let bag = OrderedBag::from(vec![
            ("key1", "value1"),
            ("key2", "value2"),
            ("key1", "value3"),
        ]);

        assert_eq!(bag.get("key1"), Some(&"value1"));
        assert_eq!(bag.get("key2"), Some(&"value2"));
    }

    #[test]
    fn test_ordered_bag_get_with_nonexistent_key() {
        let bag = OrderedBag::from(vec![
            ("key1", "value1"),
            ("key2", "value2"),
            ("key1", "value3"),
        ]);

        assert_eq!(bag.get("key3"), None);
    }

    #[test]
    fn test_ordered_bag_get_all() {
        let bag = OrderedBag::from(vec![
            ("key1", "value1"),
            ("key2", "value2"),
            ("key1", "value3"),
        ]);

        assert_eq!(bag.get_all("key1"), [&"value1", &"value3"].to_vec());
        assert_eq!(bag.get_all("key2"), [&"value2"].to_vec());
    }

    #[test]
    fn test_ordered_bag_get_all_with_nonexistent_key() {
        let bag = OrderedBag::from(vec![
            ("key1", "value1"),
            ("key2", "value2"),
            ("key1", "value3"),
        ]);

        assert!(bag.get_all("key3").is_empty());
    }

    #[test]
    fn test_ordered_bag_insert() {
        let mut bag = OrderedBag::new();
        bag.insert("key1", "value1");
        bag.insert("key2", "value2");
        bag.insert("key1", "value3");

        assert_eq!(bag.get("key1"), Some(&"value1"));
        assert_eq!(bag.get("key2"), Some(&"value2"));
    }

    #[test]
    fn test_ordered_bag_iter() {
        let mut bag = OrderedBag::new();
        bag.insert("key1", "value1");
        bag.insert("key2", "value2");
        bag.insert("key1", "value3");

        let mut iter = bag.iter();
        assert_eq!(iter.next(), Some(&("key1", "value1")));
        assert_eq!(iter.next(), Some(&("key2", "value2")));
        assert_eq!(iter.next(), Some(&("key1", "value3")));
    }

    #[test]
    fn test_ordered_bag_iter_from_vec() {
        let bag = OrderedBag::from(vec![
            ("key1", "value1"),
            ("key2", "value2"),
            ("key1", "value3"),
        ]);

        let mut iter = bag.iter();
        assert_eq!(iter.next(), Some(&("key1", "value1")));
        assert_eq!(iter.next(), Some(&("key2", "value2")));
        assert_eq!(iter.next(), Some(&("key1", "value3")));
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_ordered_bag_into_iter() {
        let mut bag = OrderedBag::new();
        bag.insert("key1", "value1");
        bag.insert("key2", "value2");
        bag.insert("key1", "value3");

        let mut iter = bag.into_iter();
        assert_eq!(iter.next(), Some(("key1", "value1")));
        assert_eq!(iter.next(), Some(("key2", "value2")));
        assert_eq!(iter.next(), Some(("key1", "value3")));
        assert!(iter.next().is_none());
    }
}
