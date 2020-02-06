use crate::Number;

impl From<i8> for Number {
    fn from(i: i8) -> Self {
        Number::Integer8(i)
    }
}

impl From<i16> for Number {
    fn from(i: i16) -> Self {
        Number::Integer16(i)
    }
}

impl From<i32> for Number {
    fn from(i: i32) -> Self {
        Number::Integer32(i)
    }
}

impl From<i64> for Number {
    fn from(i: i64) -> Self {
        Number::Integer64(i)
    }
}

impl From<i128> for Number {
    fn from(i: i128) -> Self {
        Number::Integer128(i)
    }
}

impl From<u8> for Number {
    fn from(u: u8) -> Self {
        Number::Unsigned8(u)
    }
}

impl From<u16> for Number {
    fn from(u: u16) -> Self {
        Number::Unsigned16(u)
    }
}

impl From<u32> for Number {
    fn from(u: u32) -> Self {
        Number::Unsigned32(u)
    }
}

impl From<u64> for Number {
    fn from(u: u64) -> Self {
        Number::Unsigned64(u)
    }
}

impl From<u128> for Number {
    fn from(u: u128) -> Self {
        Number::Unsigned128(u)
    }
}

impl From<f32> for Number {
    fn from(d: f32) -> Self {
        Number::Decimal32(d)
    }
}

impl From<f64> for Number {
    fn from(d: f64) -> Self {
        Number::Decimal64(d)
    }
}
