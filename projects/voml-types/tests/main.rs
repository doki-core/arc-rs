use std::{env::current_dir, path::PathBuf};
use std::env::set_current_dir;

use num::FromPrimitive;
use peginator_codegen::Compile;
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
    Tuple(u8, u8),
    Struct { x: u8, y: u8 },
}

#[derive(Serialize, Deserialize)]
enum TestEnum2 {
    T(u8, u8),
    U(String, u32, u32),
}
