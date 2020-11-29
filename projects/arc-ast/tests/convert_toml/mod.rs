use arc_ast::Value;

use std::fs;
use arc_ast::utils::parse_toml;

#[test]
fn main() {
    let json = include_str!("example.toml");
    let v  = parse_toml(json).unwrap();
    println!("{:#?}", v)
}

fn test_toml(name: &str) {
    let input = format!("tests/convert_toml/{}.toml", name);
    let output = format!("tests/convert_toml/out/{}.arc", name);
    let toml = fs::read_to_string(input).unwrap();
    let out: toml::Value = toml::from_str(&toml).unwrap();
    fs::write(output, format!("{:#?}", Value::from(out))).unwrap()
}

#[test]
fn test_hard() {
    test_toml("example");
    test_toml("hard");
    test_toml("hard_unicode");
}
