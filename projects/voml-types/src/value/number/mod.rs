use super::*;

mod cmp;

impl FromPrimitive for Von {
    #[inline]
    fn from_isize(n: isize) -> Option<Self> {
        Some(Von::Integer(Box::new(Integer::from_isize(n)?)))
    }
    #[inline]
    fn from_i8(n: i8) -> Option<Self> {
        Some(Von::Integer(Box::new(Integer::from_i8(n)?)))
    }
    #[inline]
    fn from_i16(n: i16) -> Option<Self> {
        Some(Von::Integer(Box::new(Integer::from_i16(n)?)))
    }
    #[inline]
    fn from_i32(n: i32) -> Option<Self> {
        Some(Von::Integer(Box::new(Integer::from_i32(n)?)))
    }
    #[inline]
    fn from_i64(n: i64) -> Option<Self> {
        Some(Von::Integer(Box::new(Integer::from_i64(n)?)))
    }
    #[inline]
    fn from_i128(n: i128) -> Option<Self> {
        Some(Von::Integer(Box::new(Integer::from_i128(n)?)))
    }
    #[inline]
    fn from_usize(n: usize) -> Option<Self> {
        Some(Von::Integer(Box::new(Integer::from_usize(n)?)))
    }
    #[inline]
    fn from_u8(n: u8) -> Option<Self> {
        Some(Von::Integer(Box::new(Integer::from_u8(n)?)))
    }
    #[inline]
    fn from_u16(n: u16) -> Option<Self> {
        Some(Von::Integer(Box::new(Integer::from_u16(n)?)))
    }
    #[inline]
    fn from_u32(n: u32) -> Option<Self> {
        Some(Von::Integer(Box::new(Integer::from_u32(n)?)))
    }
    #[inline]
    fn from_u64(n: u64) -> Option<Self> {
        Some(Von::Integer(Box::new(Integer::from_u64(n)?)))
    }
    #[inline]
    fn from_u128(n: u128) -> Option<Self> {
        Some(Von::Integer(Box::new(Integer::from_u128(n)?)))
    }
    #[inline]
    fn from_f32(n: f32) -> Option<Self> {
        Some(Von::Decimal(Box::new(Decimal::from_f32(n)?)))
    }
    #[inline]
    fn from_f64(n: f64) -> Option<Self> {
        Some(Von::Decimal(Box::new(Decimal::from_f64(n)?)))
    }
}

impl ToPrimitive for Von {
    #[inline]
    fn to_isize(&self) -> Option<isize> {
        match self {
            Von::Integer(v) => v.to_isize(),
            Von::Decimal(v) => v.to_isize(),
            _ => None,
        }
    }
    #[inline]
    fn to_i8(&self) -> Option<i8> {
        match self {
            Von::Integer(v) => v.to_i8(),
            Von::Decimal(v) => v.to_i8(),
            _ => None,
        }
    }
    #[inline]
    fn to_i16(&self) -> Option<i16> {
        match self {
            Von::Integer(v) => v.to_i16(),
            Von::Decimal(v) => v.to_i16(),
            _ => None,
        }
    }
    #[inline]
    fn to_i32(&self) -> Option<i32> {
        match self {
            Von::Integer(v) => v.to_i32(),
            Von::Decimal(v) => v.to_i32(),
            _ => None,
        }
    }
    #[inline]
    fn to_i64(&self) -> Option<i64> {
        match self {
            Von::Integer(v) => v.to_i64(),
            Von::Decimal(v) => v.to_i64(),
            _ => None,
        }
    }
    #[inline]
    fn to_i128(&self) -> Option<i128> {
        match self {
            Von::Integer(v) => v.to_i128(),
            Von::Decimal(v) => v.to_i128(),
            _ => None,
        }
    }
    #[inline]
    fn to_usize(&self) -> Option<usize> {
        match self {
            Von::Integer(v) => v.to_usize(),
            Von::Decimal(v) => v.to_usize(),
            _ => None,
        }
    }
    #[inline]
    fn to_u8(&self) -> Option<u8> {
        match self {
            Von::Integer(v) => v.to_u8(),
            Von::Decimal(v) => v.to_u8(),
            _ => None,
        }
    }
    #[inline]
    fn to_u16(&self) -> Option<u16> {
        match self {
            Von::Integer(v) => v.to_u16(),
            Von::Decimal(v) => v.to_u16(),
            _ => None,
        }
    }
    #[inline]
    fn to_u32(&self) -> Option<u32> {
        match self {
            Von::Integer(v) => v.to_u32(),
            Von::Decimal(v) => v.to_u32(),
            _ => None,
        }
    }
    #[inline]
    fn to_u64(&self) -> Option<u64> {
        match self {
            Von::Integer(v) => v.to_u64(),
            Von::Decimal(v) => v.to_u64(),
            _ => None,
        }
    }
    #[inline]
    fn to_u128(&self) -> Option<u128> {
        match self {
            Von::Integer(v) => v.to_u128(),
            Von::Decimal(v) => v.to_u128(),
            _ => None,
        }
    }
    #[inline]
    fn to_f32(&self) -> Option<f32> {
        match self {
            Von::Integer(v) => v.to_f32(),
            Von::Decimal(v) => v.to_f32(),
            _ => None,
        }
    }
    #[inline]
    fn to_f64(&self) -> Option<f64> {
        match self {
            Von::Integer(v) => v.to_f64(),
            Von::Decimal(v) => v.to_f64(),
            _ => None,
        }
    }
}

impl From<bool> for Von {
    fn from(value: bool) -> Self {
        Von::Boolean(value)
    }
}