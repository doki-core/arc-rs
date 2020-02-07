extern crate arc_number;

use arc_number::Number;
use std::convert::TryInto;

#[test]
fn byte_parse() {
    let x = Number::parse("x0", "1111").unwrap();
    let o = Number::parse("o0", "1111").unwrap();
    let b = Number::parse("b0", "1111").unwrap();
    println!("{}", x);
    println!("{}", o);
    println!("{}", b);
}

#[test]
fn add() {
    let x: i32 = Number::Integer8(100).into();
    println!("{}", x);
}