use crate::{BigInt, Number};
use bigdecimal::BigDecimal;
use num::Num;
use std::convert::TryFrom;

pub fn parse_integer(text: &str) -> Option<Number> {
    match BigInt::from_str_radix(text, 10) {
        Ok(i) => match usize::try_from(i.clone()) {
            Ok(u) => Some(Number::from(u)),
            Err(_) => Some(Number::from(i)),
        },
        Err(_) => None,
    }
}

pub fn parse_decimal(text: &str) -> Option<Number> {
    BigDecimal::from_str_radix(text, 10)
        .map(|e| Number::from(e))
        .ok()
}
