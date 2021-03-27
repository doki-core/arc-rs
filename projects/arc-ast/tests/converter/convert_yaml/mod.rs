use arc_ast::{utils::parse_yaml, Result};
use std::fs::{self, read_to_string};

fn test_yaml(name: &str) -> Result<()> {
    let input = format!("tests/convert_yaml/{}.yaml", name);
    let output = format!("tests/convert_yaml/out/{}.arc", name);
    let out = parse_yaml(&read_to_string(input)?)?;
    fs::write(output, format!("{:#?}", out))?;
    Ok(())
}

#[test]
fn test_easy() -> Result<()> {
    test_yaml("easy_1")?;
    test_yaml("easy_2")?;
    test_yaml("easy_3")?;
    test_yaml("easy_4")?;
    test_yaml("easy_5")?;
    Ok(())
}

#[test]
fn test_normal() -> Result<()> {
    // test_yaml("normal_1")?;
    test_yaml("normal_2")?;
    test_yaml("normal_3")?;
    test_yaml("normal_4")?;
    test_yaml("normal_5")?;
    Ok(())
}

#[test]
fn test_hard() -> Result<()> {
    test_yaml("hard_1")?;
    Ok(())
}
