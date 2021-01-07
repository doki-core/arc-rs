use super::*;

#[test]
fn test() {
    let ast = parse("tests/hard_structure/nested_key.arc").unwrap();
    println!("{:#?}", Value::from(ast))
}
