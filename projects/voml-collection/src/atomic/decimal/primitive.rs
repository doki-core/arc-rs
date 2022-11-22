use num::ToPrimitive;

use super::*;

impl FromPrimitive for Decimal {
    fn from_isize(n: isize) -> Option<Self> {
        Some(Decimal { hint: "".to_string(), value: BigDecimal::from_isize(n)? })
    }

    fn from_i8(n: i8) -> Option<Self> {
        Some(Decimal { hint: "".to_string(), value: BigDecimal::from_i8(n)? })
    }

    fn from_i16(n: i16) -> Option<Self> {
        Some(Decimal { hint: "".to_string(), value: BigDecimal::from_i16(n)? })
    }

    fn from_i32(n: i32) -> Option<Self> {
        Some(Decimal { hint: "".to_string(), value: BigDecimal::from_i32(n)? })
    }

    fn from_i64(n: i64) -> Option<Self> {
        Some(Decimal { hint: "".to_string(), value: BigDecimal::from_i64(n)? })
    }

    fn from_i128(n: i128) -> Option<Self> {
        Some(Decimal { hint: "".to_string(), value: BigDecimal::from_i128(n)? })
    }

    fn from_usize(n: usize) -> Option<Self> {
        Some(Decimal { hint: "".to_string(), value: BigDecimal::from_usize(n)? })
    }

    fn from_u8(n: u8) -> Option<Self> {
        Some(Decimal { hint: "".to_string(), value: BigDecimal::from_u8(n)? })
    }

    fn from_u16(n: u16) -> Option<Self> {
        Some(Decimal { hint: "".to_string(), value: BigDecimal::from_u16(n)? })
    }

    fn from_u32(n: u32) -> Option<Self> {
        Some(Decimal { hint: "".to_string(), value: BigDecimal::from_u32(n)? })
    }

    fn from_u64(n: u64) -> Option<Self> {
        Some(Decimal { hint: "".to_string(), value: BigDecimal::from_u64(n)? })
    }

    fn from_u128(n: u128) -> Option<Self> {
        Some(Decimal { hint: "".to_string(), value: BigDecimal::from_u128(n)? })
    }

    fn from_f32(n: f32) -> Option<Self> {
        Some(Decimal { hint: "".to_string(), value: BigDecimal::from_f32(n)? })
    }

    fn from_f64(n: f64) -> Option<Self> {
        Some(Decimal { hint: "".to_string(), value: BigDecimal::from_f64(n)? })
    }
}

impl ToPrimitive for Decimal {
    fn to_isize(&self) -> Option<isize> {
        todo!()
    }

    fn to_i8(&self) -> Option<i8> {
        todo!()
    }

    fn to_i16(&self) -> Option<i16> {
        todo!()
    }

    fn to_i32(&self) -> Option<i32> {
        todo!()
    }

    fn to_i64(&self) -> Option<i64> {
        todo!()
    }

    fn to_i128(&self) -> Option<i128> {
        todo!()
    }

    fn to_usize(&self) -> Option<usize> {
        todo!()
    }

    fn to_u8(&self) -> Option<u8> {
        todo!()
    }

    fn to_u16(&self) -> Option<u16> {
        todo!()
    }

    fn to_u32(&self) -> Option<u32> {
        todo!()
    }

    fn to_u64(&self) -> Option<u64> {
        todo!()
    }

    fn to_u128(&self) -> Option<u128> {
        todo!()
    }

    fn to_f32(&self) -> Option<f32> {
        todo!()
    }

    fn to_f64(&self) -> Option<f64> {
        todo!()
    }
}
