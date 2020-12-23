use super::*;
use arc_number::Number;

macro_rules! wrap_number {
    ($T:ty) => {
    impl From<$T> for Value {
        fn from(n: $T) -> Self {
           Self::Number(Box::new(n.into()))
        }
    }
    };
    ($($T:ty), +) => {
        $(wrap_number!($T);)+
    };
}

wrap_number![BigInt, BigUint, BigDecimal];
wrap_number![f32, f64];
wrap_number![u8, u16, u32, u64, u128, usize];
wrap_number![i8, i16, i32, i64, i128, isize];

impl From<Number> for Value {
    fn from(v: Number) -> Self {
        Self::Number(Box::new(v))
    }
}
