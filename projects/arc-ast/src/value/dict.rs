use crate::Value;
use std::collections::{BTreeMap, HashMap};
use indexmap::map::IndexMap;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Dict {
    handler: Option<String>,
    value: IndexMap<String, Value>,
}

macro_rules! native2dict {
    ($T:ty) => {
    impl<K, V> From<$T> for Dict
    where
        K: Into<String>,
        V: Into<Value>,
    {
        fn from(input: $T) -> Self {
            let mut dict = IndexMap::new();
            for (k, v) in input.into_iter() {
                dict.insert(k.into(), v.into());
            }
            Self { handler: None, value: dict }
        }
    }
    };
    ($($T:ty), +) => {
        $(wrap_native!($T);)+
    };
}

macro_rules! native2value {
    ($T:ty) => {
    native2dict!($T);
    impl<K, V> From<$T> for Value
    where
        K: Into<String>,
        V: Into<Value>,
    {
        fn from(v: $T) -> Self {
            Self::Dict(Box::new(v.into()))
        }
    }
    };
    ($($T:ty), +) => {
        $(native2value!($T);)+
    };
}

native2value![IndexMap<K, V>, HashMap<K, V>];

impl<K, V> From<BTreeMap<K, V>> for Dict
where
    K: Into<String>,
    V: Into<Value>,
{
    fn from(input: BTreeMap<K, V>) -> Self {
        let mut dict = IndexMap::new();
        for (k, v) in input.into_iter() {
            dict.insert(k.into(), v.into());
        }
        Self { handler: None, value: dict }
    }
}

impl<K, V> From<BTreeMap<K, V>> for Value
where
    K: Into<String>,
    V: Into<Value>,
{
    fn from(v: BTreeMap<K, V>) -> Self {
        Self::Dict(Box::new(v.into()))
    }
}
