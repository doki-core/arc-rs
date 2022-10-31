use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dict<T> {
    pub hint: String,
    pub dict: IndexMap<String, T>,
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
