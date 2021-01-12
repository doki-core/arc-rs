use super::*;

#[test]
fn test() {
    let ast = parse("tests/hard_structure/nested_key.arc").unwrap();
    println!("{:#?}", Value::from(ast))
}

#[test]
fn negative_key() {
    let ast = parse("tests/hard_structure/negative_key.arc").unwrap();
    println!("{:#?}", Value::from(ast))
}
