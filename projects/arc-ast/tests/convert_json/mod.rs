use arc_ast::Value;

#[test]
fn json() {
    let json = include_str!("test.json");
    let v: serde_json::Value = serde_json::from_str(json).unwrap();
    println!("{:#?}", Value::from(v))
}
