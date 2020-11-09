mod number_from;
// mod number_into;
// pub mod number_impl;
// pub mod number_traits;

use num::{BigInt, BigUint};
use bigdecimal::BigDecimal;
use std::fmt::{self, Formatter, Debug, Display};

#[derive(Debug, Clone, PartialEq)]
pub enum NumberKind {
    InlineInteger(usize),
    InlineDecimal(f64),
    BigInteger(BigInt),
    BigDecimal(BigDecimal),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Number {
    handler: Option<String>,
    value: NumberKind,
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.handler {
            Some(s) => write!(f, "{:?}{}", self.value, s),
            None => write!(f, "{:?}", self.value),
        }
    }
}

impl Eq for NumberKind  {}

#[test]
fn test_size() {
    assert_eq!(std::mem::size_of::<f64>(), 8);
    assert_eq!(std::mem::size_of::<u64>(), 8);
    assert_eq!(std::mem::size_of::<u128>(), 16);
    assert_eq!(std::mem::size_of::<BigInt>(), 32);
    assert_eq!(std::mem::size_of::<BigDecimal>(), 40);
    assert_eq!(std::mem::size_of::<NumberKind>(), 48);
    assert_eq!(std::mem::size_of::<Number>(), 72);
}