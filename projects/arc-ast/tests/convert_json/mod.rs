use arc_ast::Value;

const JSON_TEST: &str = r#"
{
   "null": null,
   "true": true,
   "false": false
}
"#;

#[test]
fn json() {
    let v: serde_json::Value = serde_json::from_str(JSON_TEST).unwrap();
    println!("{:?}", Value::from(v))
}
