use super::*;

#[test]
fn test() {
    let ast = parse("tests/json_compatibility/package.json").unwrap();
    println!("{:?}", ast)
}
