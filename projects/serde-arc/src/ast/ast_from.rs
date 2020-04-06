use crate::Arc;
use arc_number::Number;

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
