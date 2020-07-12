use arc_ast::Value;

#[test]
fn json() {
    let json = include_str!("example.toml");
    let v: toml::Value = toml::from_str(json).unwrap();
    println!("{:#?}", Value::from(v))
}
