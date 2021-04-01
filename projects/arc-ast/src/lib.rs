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
#[macro_use]
mod macros;

pub use ast::{AST,ASTKind};
pub use errors::{Result, ReadableConfigError};
pub use parser::ParserConfig;
pub use value::Value;
pub use lsp_types::Range;

/// if ture, { } will be null
pub const BUILD_EMPTY_SCOPE: bool = true;