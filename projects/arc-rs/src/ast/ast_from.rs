use crate::{
    ast::{KeyNode, KeyPath},
    Arc,
};
use arc_number::Number;
use std::collections::VecDeque;

impl From<bool> for Arc {
    fn from(b: bool) -> Self {
        Arc::Boolean(b)
    }
}

#[macro_export]
macro_rules! from_number {
    ($t:ty) => {
    impl From<$t> for Arc {
        fn from(i: $t) -> Self {
            Arc::Number(Number::from(i))
        }
    }
    };
    ($($t:ty),*) => {$(from_number!($t);)*};
}

from_number!(f32, f64);
from_number!(u8, u16, u32, u64, u128);
from_number!(i8, i16, i32, i64, i128);

impl From<&str> for Arc {
    fn from(s: &str) -> Self {
        Arc::String(String::from(s))
    }
}

impl From<String> for Arc {
    fn from(s: String) -> Self {
        Arc::String(s)
    }
}

impl<T> From<Option<T>> for Arc
where
    Arc: From<T>,
{
    fn from(o: Option<T>) -> Self {
        match o {
            None => Arc::Null,
            Some(i) => Arc::from(i),
        }
    }
}

impl<T> From<Vec<T>> for Arc
where
    Arc: From<T>,
{
    fn from(v: Vec<T>) -> Self {
        let mut dv = VecDeque::with_capacity(v.len());
        for i in v {
            dv.push_back(i.into())
        }
        Arc::List(dv)
    }
}

impl From<String> for KeyPath {
    fn from(s: String) -> Self {
        Self(vec![KeyNode::Key(Box::from(s))])
    }
}
