use super::*;

#[derive(Clone, Eq, PartialEq)]
pub struct List {
    handler: Option<String>,
    value: Vec<Value>,
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

macro_rules! native2list {
    ($T:ty) => {
    impl<V> From<$T> for List
    where
        V: Into<Value>,
    {
        fn from(input: $T) -> Self {
            let mut list = vec![];
            for i in input.into_iter() {
                list.push(i.into())
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
