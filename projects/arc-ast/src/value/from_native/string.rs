use crate::Value;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Text {
    handler: Option<String>,
    value: String,
}

macro_rules! native2string {
    ($T:ty) => {
    impl From<$T> for Text {
        fn from(v: $T) -> Self {
            Self {
                handler: None,
                value: String::from(v)
            }
        }
    }
    };
    ($($T:ty), +) => {
        $(wrap_native!($T);)+
    };
}

macro_rules! native2value {
    ($T:ty) => {
    native2string!($T);
    impl From<$T> for Value {
        fn from(v: $T) -> Self {
            Value::String(Box::new(v.into()))
        }
    }
    };
    ($($T:ty), +) => {
        $(native2value!($T);)+
    };
}

native2value![char, &str, String, &String];

impl From<Text> for Value {
    fn from(v: Text) -> Self {
        Value::String(Box::new(v))
    }
}
