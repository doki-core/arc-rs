// pub mod number_from;
// pub mod number_impl;
// pub mod number_traits;

use num::{BigInt, BigUint};
use bigdecimal::BigDecimal;

// #[derive(Clone, PartialEq)]
#[derive(Clone,Eq, PartialEq)]
pub enum Number {
    Integer(BigInt),
    Integer8(i8),
    Integer16(i16),
    Integer32(i32),
    Integer64(i64),
    Integer128(i128),
    Unsigned(BigUint),
    Unsigned8(u8),
    Unsigned16(u16),
    Unsigned32(u32),
    Unsigned64(u64),
    Unsigned128(u128),
    Decimal(BigDecimal),
    Decimal32([u8;4]),
    Decimal64([u8;8]),
}

#[test]
fn test_size() {
    assert_eq!(std::mem::size_of::<f64>(), 8);
    assert_eq!(std::mem::size_of::<u64>(), 8);
    assert_eq!(std::mem::size_of::<i128>(), 16);
    assert_eq!(std::mem::size_of::<BigInt>(), 32);
    assert_eq!(std::mem::size_of::<BigDecimal>(), 40);
    assert_eq!(std::mem::size_of::<Number>(), 40);
}