extern crate arc_number;

use arc_number::Number;

#[test]
fn test() {
    let s = Number::parse("x0", "FFF");
    println!("{:?}", s.unwrap_err())
}
