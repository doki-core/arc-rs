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