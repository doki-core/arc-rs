use arc_ast::{Value, Result};

use std::fs::{self, read_to_string};
use arc_ast::utils::parse_toml;


fn test_toml(name: &str) -> Result<()> {
    let input = format!("tests/convert_toml/{}.toml", name);
    let output = format!("tests/convert_toml/out/{}.arc", name);
    let out =  parse_toml( &read_to_string(input)?)?;
    fs::write(output, format!("{:#?}", Value::from(out)))?;
    Ok(())
}

#[test]
fn test_hard()-> Result<()> {
    test_toml("example")?;
    test_toml("hard")?;
    test_toml("hard_unicode")?;
    Ok(())
}
