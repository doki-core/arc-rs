#![feature(box_syntax)]
#![warn(missing_docs)]

//! missing


mod ast;
mod parser;
mod serde;
mod traits;
pub mod utils;
pub mod value;
mod convert;
mod errors;
mod function;
#[macro_use]
mod macros;

pub use ast::{TextRange, AST};
pub use errors::{Result, ReadableConfigError};
pub use parser::ParserConfig;
pub use value::Value;
