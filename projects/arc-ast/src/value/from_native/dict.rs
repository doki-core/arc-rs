use super::*;

#[derive(Clone, Eq, PartialEq)]
pub struct Dict {
    handler: Option<String>,
    value: IndexMap<String, Value>,
}

impl Debug for Dict {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.handler {
            Some(s) => write!(f, "{}", s)?,
            None => (),
        }
        Debug::fmt(&self.value, f)
    }
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

native2value![IndexMap<K, V>, HashMap<K, V>, BTreeMap<K, V>];

impl From<Dict> for Value {
    fn from(v: Dict) -> Self {
        Value::Dict(Box::new(v))
    }
}
