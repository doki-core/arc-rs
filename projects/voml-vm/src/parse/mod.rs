use crate::{BigInt, Number};
use bigdecimal::BigDecimal;
use num::Num;
use std::{convert::TryFrom, str::FromStr};

#[inline]
pub fn parse_integer(text: &str) -> Option<Number> {
    match BigInt::from_str_radix(text, 10) {
        Ok(i) => match usize::try_from(i.clone()) {
            Ok(u) => Some(Number::from(u)),
            Err(_) => Some(Number::from(i)),
        },
        Err(_) => None,
    }
}

#[inline]
pub fn parse_decimal(text: &str) -> Option<Number> {
    BigDecimal::from_str_radix(text, 10).map(|e| Number::from(e)).ok()
}

#[inline]
pub fn parse_number(s: &str) -> Option<Number> {
    let exp_separator: &[_] = &['e', 'E', '*'];
    let (base_part, exponent_value) = match s.find(exp_separator) {
        None => (s, 0),
        Some(loc) => {
            let (base, exp) = (&s[..loc], &s[loc + 1..]);
            let exp = match exp.chars().next() {
                Some('+') => &exp[1..],
                _ => exp,
            };
            match i64::from_str(exp.trim_start_matches('*')) {
                Ok(o) => (base, o),
                Err(_) => return None,
            }
        }
    };
    if base_part == "" {
        return None;
    }
    let (digits, decimal_offset): (String, _) = match base_part.find('.') {
        None => {
            if exponent_value >= 0 {
                return match BigInt::from_str_radix(&base_part, 10).map(|i| i * 10_u32.pow(exponent_value as u32)) {
                    Ok(i) => match usize::try_from(i.clone()) {
                        Ok(u) => Some(Number::from(u)),
                        Err(_) => Some(Number::from(i)),
                    },
                    Err(_) => None,
                };
            }
            else {
                (base_part.to_string(), 0)
            }
        }
        Some(loc) => {
            let (lead, trail) = (&base_part[..loc], &base_part[loc + 1..]);
            let mut digits = String::from(lead);
            digits.push_str(trail);
            (digits, trail.len() as i64)
        }
    };

    BigInt::from_str_radix(&digits, 10)
        .map(|big_int| BigDecimal::new(big_int, decimal_offset - exponent_value))
        .map(|i| Number::from(i))
        .ok()
}

#[test]
fn test() {
    println!("{:?}", parse_number("+2**5").unwrap());
    println!("{:?}", parse_number("+2.e5").unwrap());
}
