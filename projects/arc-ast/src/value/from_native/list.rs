use super::*;
use std::{slice::Iter, str::FromStr, vec::IntoIter};
use std::collections::btree_map::Values;

#[derive(Clone, Eq, PartialEq)]
pub struct List {
    handler: Option<String>,
    value: BTreeMap<usize, Value>,
}

impl Debug for List {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.handler {
            Some(s) => write!(f, "{}", s)?,
            None => (),
        }
        Debug::fmt(&self.value, f)
    }
}

impl Default for List {
    fn default() -> Self {
        Self { handler: None, value: Default::default() }
    }
}

macro_rules! native2list {
    ($T:ty) => {
    impl<V> From<$T> for List
    where
        V: Into<Value>,
    {
        fn from(input: $T) -> Self {
            let mut list = BTreeMap::new();

            for (i, v) in input.into_iter().enumerate() {
                list.insert(i, Value::from(v))
            }
            Self { handler: None, value: list }
        }
    }
    };
    ($($T:ty), +) => {
        $(native2list!($T);)+
    };
}

macro_rules! native2value {
    ($T:ty) => {
    native2list!($T);
    impl<V> From<$T> for Value
    where
        V: Into<Value>,
    {
        fn from(v: $T) -> Self {
            Self::List(Box::new(v.into()))
        }
    }
    };
    ($($T:ty), +) => {
        $(native2value!($T);)+
    };
}

native2value![Vec<V>, VecDeque<V>, LinkedList<V>, HashSet<V>, BTreeSet<V>];

impl From<List> for Value {
    fn from(v: List) -> Self {
        Value::List(Box::new(v))
    }
}

// impl Index<usize> for List {
//     type Output = Value;
//     fn index(&self, n: usize) -> &Self::Output {
//         self.value.index(n)
//     }
// }

impl List {
    pub fn empty() -> Value {
        Value::from(List::default())
    }

    pub fn as_iter(&self) -> Values<'_, usize, Value> {
        self.value.values()
    }

    pub fn length(&self) -> usize {
        self.value.len()
    }

    pub fn as_vec(&self) -> Vec<Value> {
        self.value.values().collect()
    }

    pub fn get_handler(&self) -> Option<String> {
        self.handler.to_owned()
    }
    pub fn get(&self, index: &str) -> Option<&Value> {
        let i = match isize::from_str(index) {
            Ok(o) => o,
            Err(_) => return None,
        };
        if i > 0 { self.value.get(i as usize) } else { self.value.get((self.value.len() as isize + i) as usize) }
    }

    pub fn extend(&mut self, item: impl Into<List>) {
        self.value.extend(item.into().value)
    }
    pub fn extend_one(&mut self, item: impl Into<Value>) {
       unimplemented!()

    }
}
