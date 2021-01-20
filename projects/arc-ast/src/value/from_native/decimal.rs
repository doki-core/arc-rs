use super::*;


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Decimal {
    handler: Option<String>,
    value: BigDecimal,
}

macro_rules! native2decimal {
    ($T:ty) => {
    impl From<$T> for Decimal {
        fn from(v: $T) -> Self {
            Self {
                handler: None,
                value: BigDecimal::from(v),
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
    native2decimal!($T);
    impl From<$T> for Value {
        fn from(v: $T) -> Self {
            Value::Decimal(Box::new(v.into()))
        }
    }
    };
    ($($T:ty), +) => {
        $(native2value!($T);)+
    };
}

native2value![f32, f64, BigDecimal];