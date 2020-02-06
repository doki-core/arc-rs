use num::{BigInt, Num, BigUint};
use crate::Number;
use std::num::ParseFloatError;


#[allow(unused_variables)]
impl Number {
    pub fn parse(h: &str, data: &str) -> Result<Number, &'static str> {
        let e = match h {
            "i8" => {
                let number = data.parse::<i8>().expect("Error: Parse failed");
                return Ok(Number::Integer8(number));
            }
            "i16" => {
                let number = data.parse::<i16>().expect("Error: Parse failed");
                return Ok(Number::Integer16(number));
            }
            "i32" => {
                let number = data.parse::<i32>().expect("Error: Parse failed");
                return Ok(Number::Integer32(number));
            }
            "i64" => {
                let number = data.parse::<i64>().expect("Error: Parse failed");
                return Ok(Number::Integer64(number));
            }
            "i128" => {
                let number = data.parse::<i128>().expect("Error: Parse failed");
                return Ok(Number::Integer128(number));
            }
            "u8" => {
                let number = data.parse::<u8>().expect("Error: Parse failed");
                return Ok(Number::Unsigned8(number));
            }
            "u16" => {
                let number = data.parse::<u16>().expect("Error: Parse failed");
                return Ok(Number::Unsigned16(number));
            }
            "u32" => {
                let number = data.parse::<u32>().expect("Error: Parse failed");
                return Ok(Number::Unsigned32(number));
            }
            "u64" => {
                let number = data.parse::<u64>().expect("Error: Parse failed");
                return Ok(Number::Unsigned64(number));
            }
            "u128" => {
                let number = data.parse::<u128>().expect("Error: Parse failed");
                return Ok(Number::Unsigned128(number));
            }
            "int" => {
                let number = BigInt::from_str_radix(&data, 10).expect("Error: Parse failed");
                return Ok(Number::Integer(number));
            }
            "unt" => {
                let number = BigUint::from_str_radix(&data, 10).expect("Error: Parse failed");
                return Ok(Number::Unsigned(number));
            }
            "x0" => {
                let number = BigInt::from_str_radix(&data, 16).expect("Error: Parse failed");
                return Ok(Number::Integer(number));
            }
            "o0" => {
                let number = BigInt::from_str_radix(&data, 8).expect("Error: Parse failed");
                return Ok(Number::Integer(number));
            }
            "b0" => {
                let number = BigInt::from_str_radix(&data, 2).expect("Error: Parse failed");
                return Ok(Number::Integer(number));
            }
            "f32" => {
                let number = data.parse::<f32>().expect("Error: Parse failed");
                return Ok(Number::Decimal32(number));
            }
            "f64" => {
                let number = data.parse::<f64>().expect("Error: Parse failed");
                return Ok(Number::Decimal64(number));
            }
            _ => Err("Error: Unsupported number type"),
        };
        return e;
    }
}
