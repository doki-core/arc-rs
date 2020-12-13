mod number_from;
mod number_impls;
mod number_into;
mod number_traits;

use bigdecimal::BigDecimal;
use num::Zero;
use num::{BigInt, BigUint};
use num::{FromPrimitive, ToPrimitive};
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt::{self, Debug, Display, Formatter};
use std::ops::Deref;

#[derive(Debug, Clone)]
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

#[test]
fn check_size() {
    assert_eq!(std::mem::size_of::<f64>(), 8);
    assert_eq!(std::mem::size_of::<u64>(), 8);
    assert_eq!(std::mem::size_of::<u128>(), 16);
    assert_eq!(std::mem::size_of::<BigInt>(), 32);
    assert_eq!(std::mem::size_of::<BigDecimal>(), 40);
    assert_eq!(std::mem::size_of::<NumberKind>(), 48);
    assert_eq!(std::mem::size_of::<Number>(), 72);
}
