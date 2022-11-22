use std::ops::Add;

use num::{BigInt, FromPrimitive, ToPrimitive, Zero};

use super::*;

impl From<u8> for Number {
    fn from(n: u8) -> Self {
        Number { hint: "".to_string(), value: BigDecimal::new(BigInt::from(n), 0) }
    }
}

impl FromPrimitive for Number {
    fn from_isize(n: isize) -> Option<Self> {
        Some(Number { hint: "".to_string(), value: BigDecimal::new(BigInt::from(n), 0) })
    }

    fn from_i8(n: i8) -> Option<Self> {
        Some(Number { hint: "".to_string(), value: BigDecimal::new(BigInt::from(n), 0) })
    }

    fn from_i16(n: i16) -> Option<Self> {
        Some(Number { hint: "".to_string(), value: BigDecimal::new(BigInt::from(n), 0) })
    }

    fn from_i32(n: i32) -> Option<Self> {
        Some(Number { hint: "".to_string(), value: BigDecimal::new(BigInt::from(n), 0) })
    }

    fn from_i64(n: i64) -> Option<Self> {
        Some(Number { hint: "".to_string(), value: BigDecimal::new(BigInt::from(n), 0) })
    }

    fn from_i128(n: i128) -> Option<Self> {
        Some(Number { hint: "".to_string(), value: BigDecimal::new(BigInt::from(n), 0) })
    }

    fn from_usize(n: usize) -> Option<Self> {
        Some(Number { hint: "".to_string(), value: BigDecimal::new(BigInt::from(n), 0) })
    }

    fn from_u8(n: u8) -> Option<Self> {
        Some(Number { hint: "".to_string(), value: BigDecimal::new(BigInt::from(n), 0) })
    }

    fn from_u16(n: u16) -> Option<Self> {
        Some(Number { hint: "".to_string(), value: BigDecimal::new(BigInt::from(n), 0) })
    }

    fn from_u32(n: u32) -> Option<Self> {
        Some(Number { hint: "".to_string(), value: BigDecimal::new(BigInt::from(n), 0) })
    }

    fn from_u64(n: u64) -> Option<Self> {
        Some(Number { hint: "".to_string(), value: BigDecimal::new(BigInt::from(n), 0) })
    }

    fn from_u128(n: u128) -> Option<Self> {
        Some(Number { hint: "".to_string(), value: BigDecimal::new(BigInt::from(n), 0) })
    }

    fn from_f32(n: f32) -> Option<Self> {
        Some(Number { hint: "".to_string(), value: BigDecimal::new(BigInt::from(n), 0) })
    }

    fn from_f64(n: f64) -> Option<Self> {
        Some(Number { hint: "".to_string(), value: BigDecimal::new(BigInt::from(n), 0) })
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

impl Add<Self> for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Zero for Number {
    fn zero() -> Self {
        Number { hint: "".to_string(), value: BigDecimal::zero() }
    }

    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
}
