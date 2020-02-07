use crate::Number;
use num::{BigInt, BigUint};

impl From<BigInt> for Number {
    fn from(i: BigInt) -> Self {
        Number::Integer(i)
    }
}

impl From<Number> for BigInt {
    fn from(n: Number) -> Self {
        match n {
            Number::Integer8(i) => BigInt::from(i),
            Number::Integer16(i) => BigInt::from(i),
            Number::Integer32(i) => BigInt::from(i),
            Number::Integer64(i) => BigInt::from(i),
            Number::Integer128(i) => BigInt::from(i),
            Number::Integer(i) => i,
            _ => BigInt::from(0)
        }
    }
}

impl From<BigUint> for Number {
    fn from(i: BigUint) -> Self {
        Number::Unsigned(i)
    }
}

impl From<Number> for BigUint {
    fn from(n: Number) -> Self {
        match n {
            Number::Unsigned(i) => i,
            _ => BigUint::from(0u8)
        }
    }
}


impl From<i8> for Number {
    fn from(i: i8) -> Self {
        Number::Integer8(i)
    }
}

impl From<Number> for i8 {
    fn from(n: Number) -> Self {
        match n {
            Number::Integer8(i) => i,
            _ => 0
        }
    }
}

impl From<i16> for Number {
    fn from(i: i16) -> Self {
        Number::Integer16(i)
    }
}

impl From<Number> for i16 {
    fn from(n: Number) -> Self {
        match n {
            Number::Integer8(i) => i as i16,
            Number::Integer16(i) => i,
            _ => 0
        }
    }
}

impl From<i32> for Number {
    fn from(i: i32) -> Self {
        Number::Integer32(i)
    }
}

impl From<Number> for i32 {
    fn from(n: Number) -> Self {
        match n {
            Number::Integer8(i) => i as i32,
            Number::Integer16(i) => i as i32,
            Number::Integer32(i) => i,
            _ => 0
        }
    }
}

impl From<i64> for Number {
    fn from(i: i64) -> Self {
        Number::Integer64(i)
    }
}

impl From<Number> for i64 {
    fn from(n: Number) -> Self {
        match n {
            Number::Integer8(i) => i as i64,
            Number::Integer16(i) => i as i64,
            Number::Integer32(i) => i as i64,
            Number::Integer64(i) => i,
            _ => 0
        }
    }
}

impl From<i128> for Number {
    fn from(i: i128) -> Self {
        Number::Integer128(i)
    }
}

impl From<Number> for i128 {
    fn from(n: Number) -> Self {
        match n {
            Number::Integer8(i) => i as i128,
            Number::Integer16(i) => i as i128,
            Number::Integer32(i) => i as i128,
            Number::Integer64(i) => i as i128,
            Number::Integer128(i) => i,
            _ => 0
        }
    }
}

impl From<u8> for Number {
    fn from(u: u8) -> Self {
        Number::Unsigned8(u)
    }
}

impl From<Number> for u8 {
    fn from(n: Number) -> Self {
        match n {
            Number::Unsigned8(i) => i,
            _ => 0
        }
    }
}

impl From<u16> for Number {
    fn from(u: u16) -> Self {
        Number::Unsigned16(u)
    }
}

impl From<Number> for u16 {
    fn from(n: Number) -> Self {
        match n {
            Number::Unsigned8(i) => i as u16,
            Number::Unsigned16(i) => i,
            _ => 0
        }
    }
}

impl From<u32> for Number {
    fn from(u: u32) -> Self {
        Number::Unsigned32(u)
    }
}

impl From<Number> for u32 {
    fn from(n: Number) -> Self {
        match n {
            Number::Unsigned8(i) => i as u32,
            Number::Unsigned16(i) => i as u32,
            Number::Unsigned32(i) => i,
            _ => 0
        }
    }
}


impl From<u64> for Number {
    fn from(u: u64) -> Self {
        Number::Unsigned64(u)
    }
}

impl From<Number> for u64 {
    fn from(n: Number) -> Self {
        match n {
            Number::Unsigned8(i) => i as u64,
            Number::Unsigned16(i) => i as u64,
            Number::Unsigned32(i) => i as u64,
            Number::Unsigned64(i) => i,
            _ => 0
        }
    }
}

impl From<u128> for Number {
    fn from(u: u128) -> Self {
        Number::Unsigned128(u)
    }
}

impl From<Number> for u128 {
    fn from(n: Number) -> Self {
        match n {
            Number::Unsigned8(i) => i as u128,
            Number::Unsigned16(i) => i as u128,
            Number::Unsigned32(i) => i as u128,
            Number::Unsigned64(i) => i as u128,
            Number::Unsigned128(i) => i,
            _ => 0
        }
    }
}

impl From<f32> for Number {
    fn from(d: f32) -> Self {
        Number::Decimal32(d)
    }
}

impl From<Number> for f32 {
    fn from(n: Number) -> Self {
        match n {
            Number::Decimal32(i) => i,
            _ => 0.0
        }
    }
}

impl From<f64> for Number {
    fn from(d: f64) -> Self {
        Number::Decimal64(d)
    }
}

impl From<Number> for f64 {
    fn from(n: Number) -> Self {
        match n {
            Number::Decimal32(i) => i as f64,
            Number::Decimal64(i) => i,
            _ => 0.0
        }
    }
}