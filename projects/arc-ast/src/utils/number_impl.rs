use crate::AST;
use crate::ast::{Number, null};
use num::{BigInt, Num, BigUint};


#[allow(unused_variables)]
impl Number {
    fn parse(h: &str, data: &str) -> AST {
        number_refine(h,d)
    }
}
pub fn number_refine(h: &str, data: &str) -> AST {
    let input = null;
    match h {
        "i8" => {
            let number = data.parse::<i8>().unwrap();
            return AST::Number(Number::Integer8(number));
        }
        "i16" => {
            let number = data.parse::<i16>().unwrap();
            return AST::Number(Number::Integer16(number));
        }
        "i32" => {
            let number = data.parse::<i32>().unwrap();
            return AST::Number(Number::Integer32(number));
        }
        "i64" => {
            let number = data.parse::<i64>().unwrap();
            return AST::Number(Number::Integer64(number));
        }
        "i128" => {
            let number = data.parse::<i128>().unwrap();
            return AST::Number(Number::Integer128(number));
        }
        "u8" => {
            let number = data.parse::<u8>().unwrap();
            return AST::Number(Number::Unsigned8(number));
        }
        "u16" => {
            let number = data.parse::<u16>().unwrap();
            return AST::Number(Number::Unsigned16(number));
        }
        "u32" => {
            let number = data.parse::<u32>().unwrap();
            return AST::Number(Number::Unsigned32(number));
        }
        "u64" => {
            let number = data.parse::<u64>().unwrap();
            return AST::Number(Number::Unsigned64(number));
        }
        "u128" => {
            let number = data.parse::<u128>().unwrap();
            return AST::Number(Number::Unsigned128(number));
        }
        "int" => {
            let number = BigInt::from_str_radix(&data, 10).unwrap();
            return AST::Number(Number::Integer(number));
        }
        "unt" => {
            let number = BigUint::from_str_radix(&data, 10).unwrap();
            return AST::Number(Number::Unsigned(number));
        }
        "x0" => {
            let number = BigInt::from_str_radix(&data, 16).unwrap();
            return AST::Number(Number::Integer(number));
        }
        "o0" => {
            let number = BigInt::from_str_radix(&data, 8).unwrap();
            return AST::Number(Number::Integer(number));
        }
        "b0" => {
            let number = BigInt::from_str_radix(&data, 2).unwrap();
            return AST::Number(Number::Integer(number));
        }
        "f32" => {
            let number = data.parse::<f32>().unwrap();
            return AST::Number(Number::Decimal32(number));
        }
        "f64" => {
            let number = data.parse::<f64>().unwrap();
            return AST::Number(Number::Decimal64(number));
        }
        _ => (),
    };
    return input;
}
