mod easy_structure;
mod hard_structure;
mod json_compatibility;
mod real_file;

use std::{fs::read_to_string, path::Path};

use arc_rs::{ParserConfig, Result, Value, AST};

fn parse(file: impl AsRef<Path>) -> Result<AST> {
    let parser = ParserConfig::default();
    let ast = parser.parse(&read_to_string(file.as_ref())?)?;
    Ok(ast)
}

fn parse_text(file: impl AsRef<str>) -> Result<AST> {
    let parser = ParserConfig::default();
    let ast = parser.parse(file.as_ref())?;
    Ok(ast)
}
