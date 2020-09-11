use arc_ast::Value;

use arc_ast::ast::ASTKind;

#[test]
fn main() {
    let json = include_str!("example.toml");
    let v: toml::Value = toml::from_str(json).unwrap();
    println!("{:#?}", Value::from(v))
}
