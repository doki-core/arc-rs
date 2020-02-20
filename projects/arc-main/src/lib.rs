#[macro_use]
extern crate arc_ast;

mod parse;

pub use arc_ast::{dict, list, Arc};
pub use parse::{parse, parse_file};
