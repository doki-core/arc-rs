use std::fmt::{Debug, Formatter, Write};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dict<T> {
    pub hint: String,
    pub dict: IndexMap<String, T>,
}

impl<T> Debug for Dict<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if !self.hint.is_empty() {
            f.write_str(&self.hint)?;
            f.write_char(' ')?;
        }
        f.debug_map().entries(self.dict.iter()).finish()
    }
}

impl<O, K, V> FromIterator<(K, V)> for Dict<O>
where
    O: From<V>,
    String: From<K>,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (K, V)>,
    {
        Dict { hint: "".to_string(), dict: IndexMap::from_iter(iter.into_iter().map(|(k, v)| (String::from(k), O::from(v)))) }
    }
}

impl<T> Dict<T> {
    pub fn clear(&mut self) {
        self.dict.clear()
    }
    pub fn insert<K, V>(&mut self, k: K, v: V) -> Option<T>
    where
        K: Into<String>,
        V: Into<T>,
    {
        self.dict.insert(k.into(), v.into())
    }
    /// Return a reference to the value stored for `key`, if it is present,
    /// else `None`.
    ///
    /// Computes in **O(1)** time (average).
    pub fn get_key(&self, query: &str) -> Option<&T> {
        self.dict.get(query)
    }
    /// Return a reference to the value stored for `key`, if it is present,
    /// else `None`.
    ///
    /// Computes in **O(1)** time (average).
    pub fn get_key_mut(&mut self, query: &str) -> Option<&mut T> {
        self.dict.get_mut(query)
    }
    /// Get a key-value pair by index
    ///
    /// Valid indices are *0 <= index < self.len()*
    ///
    /// Computes in **O(1)** time.
    pub fn get_index(&self, query: usize) -> Option<&T> {
        self.dict.get_index(query).map(|v| v.1)
    }
    /// Get a key-value pair by index
    ///
    /// Valid indices are *0 <= index < self.len()*
    ///
    /// Computes in **O(1)** time.
    pub fn get_index_mut(&mut self, query: usize) -> Option<&mut T> {
        self.dict.get_index_mut(query).map(|v| v.1)
    }
}
