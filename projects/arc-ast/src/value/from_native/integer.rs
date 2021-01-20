use super::*;


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Integer {
    handler: Option<String>,
    value: BigInt,
}

macro_rules! native2integer {
    ($T:ty) => {
    impl From<$T> for Integer {
        fn from(v: $T) -> Self {
            Self {
                handler: None,
                value: BigInt::from(v),
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
    native2integer!($T);
    impl From<$T> for Value {
        fn from(v: $T) -> Self {
            Value::Integer(Box::new(v.into()))
        }
    }
    };
    ($($T:ty), +) => {
        $(native2value!($T);)+
    };
}

native2value![BigInt, BigUint];
native2value![u8, u16, u32, u64, u128, usize];
native2value![i8, i16, i32, i64, i128, isize];

// wrap_number![BigInt, BigUint, BigDecimal];
// wrap_number![f32, f64];
//
//
// impl From<Integer> for Value {
//     fn from(v: Integer) -> Self {
//         Self::Integer(Box::new(v))
//     }
// }
