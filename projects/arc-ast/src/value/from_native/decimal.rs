use super::*;

#[derive(Clone, Eq, PartialEq)]
pub struct Decimal {
    handler: Option<String>,
    value: BigDecimal,
}

impl Debug for Decimal {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.handler {
            Some(s) => write!(f, "{}{}", self.value, s),
            None => write!(f, "{}", self.value),
        }
    }
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
