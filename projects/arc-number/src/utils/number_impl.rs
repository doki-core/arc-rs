use crate::Number;
use num::{BigInt, BigUint, Num};

#[allow(unused_variables)]
impl Number {
    #[inline]
    pub fn parse(h: &str, data: &str) -> Option<Number> {
        return match h {
            "i8" => match data.parse::<i8>() {
                Ok(i) => Some(Number::Integer8(i)),
                _ => None,
            },
            "i16" => match data.parse::<i16>() {
                Ok(i) => Some(Number::Integer16(i)),
                _ => None,
            },
            "i32" => match data.parse::<i32>() {
                Ok(i) => Some(Number::Integer32(i)),
                _ => None,
            },
            "i64" => match data.parse::<i64>() {
                Ok(i) => return Some(Number::Integer64(i)),
                _ => None,
            },
            "i128" => match data.parse::<i128>() {
                Ok(i) => Some(Number::Integer128(i)),
                _ => None,
            },
            "u8" => match data.parse::<u8>() {
                Ok(i) => Some(Number::Unsigned8(i)),
                _ => None,
            },
            "u16" => match data.parse::<u16>() {
                Ok(i) => Some(Number::Unsigned16(i)),
                _ => None,
            },
            "u32" => match data.parse::<u32>() {
                Ok(i) => Some(Number::Unsigned32(i)),
                _ => None,
            },
            "u64" => match data.parse::<u64>() {
                Ok(i) => Some(Number::Unsigned64(i)),
                _ => None,
            },
            "u128" => match data.parse::<u128>() {
                Ok(i) => Some(Number::Unsigned128(i)),
                _ => None,
            },
            "int" => match BigInt::from_str_radix(&data, 10) {
                Ok(i) => Some(Number::Integer(i)),
                _ => None,
            },
            "unt" => match BigUint::from_str_radix(&data, 10) {
                Ok(i) => Some(Number::Unsigned(i)),
                _ => None,
            },
            "x0" => match BigInt::from_str_radix(&data, 16) {
                Ok(i) => Some(Number::Integer(i)),
                _ => None,
            },
            "o0" => match BigInt::from_str_radix(&data, 8) {
                Ok(i) => Some(Number::Integer(i)),
                _ => None,
            },
            "b0" => match BigInt::from_str_radix(&data, 2) {
                Ok(i) => Some(Number::Integer(i)),
                _ => None,
            },
            "f32" => match data.parse::<f32>() {
                Ok(i) => Some(Number::Decimal32(i)),
                _ => None,
            },
            "f64" => match data.parse::<f64>() {
                Ok(i) => Some(Number::Decimal64(i)),
                _ => None,
            },
            _ => None,
        };
    }
}
