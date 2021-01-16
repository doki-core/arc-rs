use crate::{BigInt, Number};
use num::Num;
use bigdecimal::{BigDecimal, ParseBigDecimalError};
use num::bigint::{ParseBigIntError, TryFromBigIntError};
use std::convert::TryFrom;


pub fn parse_integer(text: &str) -> Option<Number> {
    match BigInt::from_str_radix(text, 10) {
        Ok(i) => {
            match usize::try_from(i.clone()) {
                Ok(u) => {Some(Number::from(u))}
                Err(_) => {
                    Some(Number::from(i))
                }
            }
        }
        Err(_) => {None}
    }
}

pub fn parse_decimal(text: &str) -> Option<Number> {
    match BigDecimal::from_str_radix(text, radix as u32){
        Ok(o) => {Some(o)}
        Err(_) => {None}
    }
}

