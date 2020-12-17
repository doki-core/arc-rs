use arc_rs::ParserConfig;
use arc_rs::Result;
use arc_ast::AST;
use std::fs::{self, read_to_string};
use std::path::Path;

mod json_compatibility;

fn parse(file: impl AsRef<Path>) -> Result<AST> {
    let parser = ParserConfig::default();
    let ast = parser.parse(&read_to_string(file.as_ref())?)?;
    Ok(ast)
}


#[test]
fn ready() {
    println!("it, works!")
}
