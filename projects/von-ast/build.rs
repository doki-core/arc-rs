use peginator::buildscript::Compile;
use std::{env::current_dir, path::PathBuf};

fn main() {
    let path = PathBuf::from(current_dir().unwrap());
    let output = path.join("src/parser/von.rs");
    Compile::file("src/parser/von.peg").destination(output).format().run().unwrap();
}
