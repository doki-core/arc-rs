extern crate arc_number;

use arc_number::Number;

#[test]
fn byte_parse() {
    let x = Number::parse("x0", "1111").unwrap();
    let o = Number::parse("o0", "1111").unwrap();
    let b = Number::parse("b0", "1111").unwrap();
    println!("{}", x);
    println!("{}", o);
    println!("{}", b);
}
