use crate::Number;
use num::{BigInt, BigUint, Num};
use std::fmt::{self, Display};

impl Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Integer(i) => write!(f, "{}", i),
            Number::Integer8(i) => write!(f, "{}", i),
            Number::Integer16(i) => write!(f, "{}", i),
            Number::Integer32(i) => write!(f, "{}", i),
            Number::Integer64(i) => write!(f, "{}", i),
            Number::Integer128(i) => write!(f, "{}", i),
            Number::Unsigned(i) => write!(f, "{}", i),
            Number::Unsigned8(i) => write!(f, "{}", i),
            Number::Unsigned16(i) => write!(f, "{}", i),
            Number::Unsigned32(i) => write!(f, "{}", i),
            Number::Unsigned64(i) => write!(f, "{}", i),
            Number::Unsigned128(i) => write!(f, "{}", i),
            Number::Decimal32(i) => {
                let mut s = format!("{}", i);
                if !s.ends_with('.') {
                    s.push_str(".0")
                }
                write!(f, "{}", s)
            }
            Number::Decimal64(i) => {
                let mut s = format!("{}", i);
                if !s.ends_with('.') {
                    s.push_str(".0")
                }
                write!(f, "{}", s)
            }
            _ => write!(f, "{:?}", self),
        }
    }
}
