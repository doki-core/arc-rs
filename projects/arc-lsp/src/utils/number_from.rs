use super::*;

macro_rules! cloned_integer {
    ($T:ty) => {
    impl From<$T> for NumberKind {
        fn from(i: $T) -> Self {
            Self::BigInteger(BigInt::from(i))
        }
    }
    };
    ($($T:ty ), +) => {
        $(cloned_integer!($T);)+
    };
}

macro_rules! copied_integer {
    ($T:ty) => {
    impl From<$T> for NumberKind {
        fn from(i: $T) -> Self {
            match usize::try_from(i) {
                Ok(u) => Self::InlineInteger(u),
                Err(_) => {
                    Self::BigInteger(BigInt::from(i))
                }
            }
        }
    }
    };
    ($($T:ty ), +) => {
        $(copied_integer!($T);)+
    };
}

cloned_integer![BigInt, BigUint];
copied_integer![u8, u16, u32, u64, u128, usize];
copied_integer![i8, i16, i32, i64, i128, isize];

impl From<f32> for NumberKind {
    fn from(f: f32) -> Self {
        Self::InlineDecimal(f as f64)
    }
}

impl From<f64> for NumberKind {
    fn from(f: f64) -> Self {
        Self::InlineDecimal(f)
    }
}

impl From<BigDecimal> for NumberKind {
    fn from(f: BigDecimal) -> Self {
        Self::BigDecimal(f)
    }
}
