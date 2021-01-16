use super::*;

#[test]
fn test() {
    let ast = parse("tests/easy_structure/empty.arc").unwrap();
    println!("{:#?}", Value::from(ast))
}

#[test]
fn basic() {
    let ast = parse("tests/easy_structure/basic.arc").unwrap();
    println!("{:#?}", Value::from(ast))
}
