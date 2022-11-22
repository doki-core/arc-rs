use num::FromPrimitive;
use serde::{Deserialize, Serialize};

use voml_types::Von;

mod ser;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
pub fn test() {
    assert_eq!(Von::from_usize(0).unwrap(), 0usize);
    println!("{:#?}", Von::from_f32(1.0).unwrap())
}

#[derive(Serialize, Deserialize)]
pub struct TestUnit;

#[derive(Serialize, Deserialize)]
pub struct TestTupleUnit();

#[derive(Serialize, Deserialize)]
pub struct TestTuple(u8, u16, u32);

#[derive(Serialize, Deserialize)]
pub struct TestStructUnit {}

#[derive(Serialize, Deserialize)]
pub struct TestStruct {
    a: u8,
    b: u16,
    c: u32,
}

#[derive(Serialize, Deserialize)]
pub enum TestEnumUnit {}

#[repr(u8)]
#[derive(Serialize, Deserialize)]
pub enum TestEnum {
    Nothing = 0,
    Something(bool),
}
