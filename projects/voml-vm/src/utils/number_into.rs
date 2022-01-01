use super::*;

// impl<T> From<NumberKind> for T
// where
//     T: TryFrom<NumberKind> + Default,
// {
//     fn from(n: NumberKind) -> Self {
//         T::try_from(n).unwrap_or_default()
//     }
// }

macro_rules! integer_convert {
    ($T:ty) => {
    impl TryFrom<NumberKind> for $T {
        type Error = ();
        fn try_from(n: NumberKind) -> Result<Self, Self::Error> {
            match n {
                NumberKind::BigInteger(i) => {
                    if let Ok(o) = <$T>::try_from(i) {
                        return Ok(o);
                    }
                }
                NumberKind::InlineInteger(u) => {
                    if let Ok(o) = <$T>::try_from(u) {
                        return Ok(o);
                    }
                }
                _ => (),
            }
            Err(())
        }
    }
    };
    ($($T:ty ), +) => {
        $(integer_convert!($T);)+
    };
}

integer_convert![BigInt, BigUint];
integer_convert![u8, u16, u32, u64, u128, usize];
integer_convert![i8, i16, i32, i64, i128, isize];

impl From<NumberKind> for f32 {
    fn from(n: NumberKind) -> Self {
        match n {
            NumberKind::BigInteger(i) => i.to_f32().unwrap_or_default(),
            NumberKind::InlineInteger(u) => u as f32,
            NumberKind::InlineDecimal(f) => f as f32,
            NumberKind::BigDecimal(f) => f.to_f32().unwrap_or_default(),
        }
    }
}

impl From<NumberKind> for f64 {
    fn from(n: NumberKind) -> Self {
        match n {
            NumberKind::BigInteger(i) => i.to_f64().unwrap_or_default(),
            NumberKind::InlineInteger(u) => u as f64,
            NumberKind::InlineDecimal(f) => f,
            NumberKind::BigDecimal(f) => f.to_f64().unwrap_or_default(),
        }
    }
}

impl From<&NumberKind> for f64 {
    fn from(n: &NumberKind) -> Self {
        match n {
            NumberKind::BigInteger(i) => i.to_f64().unwrap_or_default(),
            NumberKind::InlineInteger(u) => *u as f64,
            NumberKind::InlineDecimal(f) => *f,
            NumberKind::BigDecimal(f) => f.to_f64().unwrap_or_default(),
        }
    }
}

impl From<NumberKind> for BigDecimal {
    fn from(n: NumberKind) -> Self {
        match n {
            NumberKind::BigInteger(i) => BigDecimal::from(i),
            NumberKind::InlineInteger(u) => BigDecimal::from_usize(u).unwrap_or_default(),
            NumberKind::InlineDecimal(f) => BigDecimal::from_f64(f).unwrap_or_default(),
            NumberKind::BigDecimal(f) => f,
        }
    }
}
