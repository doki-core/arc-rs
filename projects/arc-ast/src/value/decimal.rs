use crate::Value;
use bigdecimal::BigDecimal;
use std::convert::TryFrom;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Decimal {
    handler: Option<String>,
    value: BigDecimal,
}

macro_rules! native2decimal {
    ($T:ty) => {
    impl From<$T> for Decimal {
        fn from(n: $T) -> Self {
            Self {
                handler: None,
                value: BigDecimal::try_from(n).unwrap_or_default()
            }
        }
    }
    };
    ($($T:ty ), +) => {
        $(native2decimal!($T);)+
    };
}

macro_rules! native2value {
    ($T:ty) => {
    impl From<$T> for Value {
        fn from(n: $T) -> Self {
            BigDecimal::try_from(n)
                .map(|value| Decimal { handler: None, value }.into())
                .unwrap_or_default()
        }
    }
    };
    ($($T:ty ), +) => {
        $(native2value!($T);)+
    };
}

native2decimal![f32, f64];
native2value![f32, f64];

impl From<Decimal> for Value {
    fn from(v: Decimal) -> Self {
        Value::Decimal(Box::new(v))
    }
}
