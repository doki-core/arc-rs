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
        let dict = IndexMap::from_iter(iter.into_iter().map(|(k, v)| (String::from(k), O::from(v))));
        Dict { hint: "".to_string(), dict }
    }
}

impl<T> Dict<T> {
    pub fn insert<K, V>(&mut self, k: K, v: V) -> Option<T>
    where
        K: Into<String>,
        V: Into<T>,
    {
        self.dict.insert(k.into(), v.into())
    }
    pub fn clear(&mut self) {
        self.dict.clear()
    }
}
