use arc_ast::Value;
use std::fs;
use yaml_rust::YamlLoader;

fn test_yaml(name: &str) {
    let input = format!("tests/convert_yaml/{}.yaml", name);
    let output = format!("tests/convert_yaml/out/{}.arc", name);
    let yaml = fs::read_to_string(input).unwrap();
    let out = YamlLoader::load_from_str(&yaml).unwrap();
    let value = match out.len() {
        1 => Value::from(out[0].clone()),
        _ => Value::from(out),
    };
    fs::write(output, format!("{:#?}", value)).unwrap()
}

#[test]
fn test_easy() {
    test_yaml("easy_1");
    test_yaml("easy_2");
    test_yaml("easy_3");
    test_yaml("easy_4");
    test_yaml("easy_5");
}

#[test]
fn test_normal() {
    // test_file("normal_1");
    test_yaml("normal_2");
    test_yaml("normal_3");
    test_yaml("normal_4");
    test_yaml("normal_5");
}

#[test]
fn test_hard() {
    test_yaml("hard_1");
}
