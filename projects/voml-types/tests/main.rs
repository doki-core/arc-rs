use num::FromPrimitive;
use voml_types::Von;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
pub fn test() {
    println!("{:#?}", Von::from_usize(0).unwrap());
    println!("{:#?}", Von::from_f32(1.0).unwrap())
}
