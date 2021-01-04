use super::*;

#[test]
fn test() {
    let ast = parse("tests/easy_structure/empty.arc").unwrap();
    println!("{:#?}", Value::from(ast))
}