use crate::Number;
use std::fmt::{self, Display};

impl Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Integer(i) => write!(f, "{}", i),
            Number::Integer8(i) => write!(f, "{}i8", i),
            Number::Integer16(i) => write!(f, "{}i16", i),
            Number::Integer32(i) => write!(f, "{}i32", i),
            Number::Integer64(i) => write!(f, "{}i64", i),
            Number::Integer128(i) => write!(f, "{}i128", i),
            Number::Unsigned(i) => write!(f, "{}", i),
            Number::Unsigned8(i) => write!(f, "{}u8", i),
            Number::Unsigned16(i) => write!(f, "{}u16", i),
            Number::Unsigned32(i) => write!(f, "{}u32", i),
            Number::Unsigned64(i) => write!(f, "{}u64", i),
            Number::Unsigned128(i) => write!(f, "{}u128", i),
            Number::Decimal32(i) => {
                let mut s = format!("{}", i);
                if !s.ends_with('.') {
                    s.push_str(".0")
                }
                write!(f, "{}f32", s)
            }
            Number::Decimal64(i) => {
                let mut s = format!("{}", i);
                if !s.ends_with('.') {
                    s.push_str(".0")
                }
                write!(f, "{}f64", s)
            }
            _ => write!(f, "{:?}", self),
        }
    }
}
