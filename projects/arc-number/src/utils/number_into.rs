
impl From<Number> for BigInt {
    fn from(n: Number) -> Self {
        match n {
            Number::Integer8(i) => BigInt::from(i),
            Number::Integer16(i) => BigInt::from(i),
            Number::Integer32(i) => BigInt::from(i),
            Number::Integer64(i) => BigInt::from(i),
            Number::Integer128(i) => BigInt::from(i),
            Number::Integer(i) => i,
            _ => BigInt::from(0),
        }
    }
}

impl From<Number> for BigUint {
    fn from(n: Number) -> Self {
        match n {
            Number::Unsigned(i) => i,
            _ => BigUint::from(0u8),
        }
    }
}

impl From<Number> for i8 {
    fn from(n: Number) -> Self {
        match n {
            Number::Integer8(i) => i,
            _ => 0,
        }
    }
}

impl From<Number> for i16 {
    fn from(n: Number) -> Self {
        match n {
            Number::Integer8(i) => i as i16,
            Number::Integer16(i) => i,
            _ => 0,
        }
    }
}

impl From<Number> for i32 {
    fn from(n: Number) -> Self {
        match n {
            Number::Integer8(i) => i as i32,
            Number::Integer16(i) => i as i32,
            Number::Integer32(i) => i,
            _ => 0,
        }
    }
}

impl From<Number> for i64 {
    fn from(n: Number) -> Self {
        match n {
            Number::Integer8(i) => i as i64,
            Number::Integer16(i) => i as i64,
            Number::Integer32(i) => i as i64,
            Number::Integer64(i) => i,
            _ => 0,
        }
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
            _ => 0,
        }
    }
}

impl From<Number> for u8 {
    fn from(n: Number) -> Self {
        match n {
            Number::Unsigned8(i) => i,
            _ => 0,
        }
    }
}
impl From<Number> for u16 {
    fn from(n: Number) -> Self {
        match n {
            Number::Unsigned8(i) => i as u16,
            Number::Unsigned16(i) => i,
            _ => 0,
        }
    }
}

impl From<Number> for u32 {
    fn from(n: Number) -> Self {
        match n {
            Number::Unsigned8(i) => i as u32,
            Number::Unsigned16(i) => i as u32,
            Number::Unsigned32(i) => i,
            _ => 0,
        }
    }
}
impl From<Number> for u64 {
    fn from(n: Number) -> Self {
        match n {
            Number::Unsigned8(i) => i as u64,
            Number::Unsigned16(i) => i as u64,
            Number::Unsigned32(i) => i as u64,
            Number::Unsigned64(i) => i,
            _ => 0,
        }
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
            _ => 0,
        }
    }
}
impl From<Number> for f32 {
    fn from(n: Number) -> Self {
        unsafe {
            match n {
                Number::Decimal32(f) => {

                        transmute::<[u8;4],f32>(f)

                },
                _ => 0.0,
            }
        }
    }
}

impl From<Number> for f64 {
    fn from(n: Number) -> Self {
        unsafe {
            match n {
                Number::Decimal32(f) => transmute::<[u8;4],f32>(f) as f64,
                Number::Decimal64(f) => transmute::<[u8;8],f64>(f),
                _ => 0.0,
            }
        }
    }
}
