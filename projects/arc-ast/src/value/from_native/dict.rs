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

impl Default for Dict {
    fn default() -> Self {
        Self { handler: None, value: IndexMap::new() }
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

impl Dict {
    pub fn empty() -> Value {
        Value::from(Dict::default())
    }

    pub fn iter(&self) -> indexmap::map::Iter<String, Value> {
        self.value.iter()
    }
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.value.get(key)
    }
    pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
        self.value.get_mut(key)
    }
    pub fn insert(&mut self, key: String, value: Value) -> Option<Value> {
        self.value.insert(key, value)
    }

    pub fn as_vec(&self) -> Vec<Value> {
        self.value.values().cloned().collect()
    }
}

// impl Deref for Dict {
//     type Target = IndexMap<String, Value>;
//
//     fn deref(&self) -> &Self::Target {
//         &self.value
//     }
// }
//
// impl DerefMut for Dict {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.value
//     }
// }
