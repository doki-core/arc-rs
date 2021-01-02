use super::*;

#[test]
fn test() {
    let ast = parse("tests/json_compatibility/package.json").unwrap();
    println!("{:#?}", Value::from(ast))
}

#[test]
fn json_string() {
    let ast = parse("tests/json_compatibility/string.json").unwrap();
    println!("{:#?}", Value::from(ast))
}


#[test]
fn json_number() {
    let ast = parse("tests/json_compatibility/number.json").unwrap();
    println!("{:#?}", Value::from(ast))
}

#[test]
fn json_object() {
    let ast = parse("tests/json_compatibility/object.json").unwrap();
    println!("{:#?}", Value::from(ast))
}

