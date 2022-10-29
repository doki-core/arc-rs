use std::str::FromStr;

use num::{BigInt, FromPrimitive};

use super::*;

impl NumCast for Number {
    fn from<T: ToPrimitive>(n: T) -> Option<Self> {
        todo!()
    }
}

impl ToPrimitive for Number {
    fn to_i64(&self) -> Option<i64> {
        self.value.to_i64()
    }

    fn to_i128(&self) -> Option<i128> {
        self.value.to_i128()
    }

    fn to_u64(&self) -> Option<u64> {
        self.value.to_u64()
    }

    fn to_u128(&self) -> Option<u128> {
        self.value.to_u128()
    }
    fn to_f64(&self) -> Option<f64> {
        self.value.to_f64()
    }
}

impl FromPrimitive for Number {
    #[inline]
    fn from_i64(n: i64) -> Option<Self> {
        Some(Number { hint: "".to_string(), value: BigDecimal::from(BigInt::from(n)) })
    }
    #[inline]
    fn from_i128(n: i128) -> Option<Self> {
        Some(Number { hint: "".to_string(), value: BigDecimal::from(BigInt::from(n)) })
    }
    #[inline]
    fn from_u64(n: u64) -> Option<Self> {
        Some(Number { hint: "".to_string(), value: BigDecimal::from(BigInt::from(n)) })
    }
    #[inline]
    fn from_u128(n: u128) -> Option<Self> {
        Some(Number { hint: "".to_string(), value: BigDecimal::from(BigInt::from(n)) })
    }
}

impl Num for Number {
    type FromStrRadixErr = ParseBigDecimalError;
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        let dec = BigDecimal::from_str_radix(str, radix)?;
        Ok(Self { hint: "".to_string(), value: dec })
    }
}

impl FromStr for Number {
    type Err = ParseBigDecimalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Number::from_str_radix(s, 10)
    }
}
