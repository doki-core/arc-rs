use arc_ast::{Value, AST};
use arc_rs::{ParserConfig, Result};
use std::{
    fs::{read_to_string},
    path::Path,
};

mod display;
mod easy_structure;
mod hard_structure;
mod json_compatibility;
mod real_file;

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

#[test]
fn ready() {
    println!("it, works!")
}
