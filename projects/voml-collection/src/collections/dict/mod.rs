use std::fmt::{Debug, Formatter, Write};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dict<T> {
    pub hint: String,
    pub dict: IndexMap<String, T>,
}

pub enum DictQuery<'i> {
    Key(&'i str),
    Index(usize),
    Hash(usize),
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
    pub fn insert<K, V>(&mut self, k: K, v: V) -> Option<T>
    where
        K: Into<String>,
        V: Into<T>,
    {
        self.dict.insert(k.into(), v.into())
    }
    pub fn get<'i, Q>(&self, query: &'i Q) -> Option<&T>
    where
        DictQuery<'i>: From<&'i Q>,
    {
        match query.into() {
            DictQuery::Key(key) => self.dict.get(key),
            DictQuery::Index(key) => self.dict.get_index(key).map(|v| v.1),
            DictQuery::Hash(_) => unimplemented!(),
        }
    }
    pub fn get_entry<'i, Q>(&self, query: &'i Q) -> Option<(usize, &str, &T)>
    where
        DictQuery<'i>: From<&'i Q>,
    {
        match query.into() {
            DictQuery::Key(key) => self.dict.get_full(key).map(|(i, k, v)| (i, k.as_str(), v)),
            DictQuery::Index(key) => self.dict.get_index(key).map(|(k, v)| (key, k.as_str(), v)),
            DictQuery::Hash(_) => unimplemented!(),
        }
    }
    pub fn clear(&mut self) {
        self.dict.clear()
    }
}
