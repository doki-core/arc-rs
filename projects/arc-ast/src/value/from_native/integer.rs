use super::*;

#[derive(Clone, Eq, PartialEq)]
pub struct Integer {
    handler: Option<String>,
    value: BigInt,
}

impl Debug for Integer {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.handler {
            Some(s) => write!(f, "{}{}", self.value, s),
            None => write!(f, "{}", self.value),
        }
    }
}

macro_rules! native2integer {
    ($T:ty) => {
    impl From<$T> for Integer {
        fn from(n: $T) -> Self {
            Self {
                handler: None,
                value: BigInt::from(n)
            }
        }
    }
    };
    ($($T:ty ), +) => {
        $(wrap_native!($T);)+
    };
}

macro_rules! native2value {
    ($T:ty) => {
    native2integer!($T);
    impl From<$T> for Value {
        fn from(n: $T) -> Self {
            Value::Integer(Box::new(n.into()))
        }
    }
    };
    ($($T:ty), +) => {
        $(native2value!($T);)+
    };
}

native2value![u8, u16, u32, u64, u128, usize];
native2value![i8, i16, i32, i64, i128, isize];

impl From<Integer> for Value {
    fn from(v: Integer) -> Self {
        Value::Integer(Box::new(v))
    }
}
