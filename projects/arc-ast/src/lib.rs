#![warn(missing_docs)]

//! missing

mod ast;
mod convert;
mod errors;
mod parser;
mod serde;
mod traits;
pub mod utils;
pub mod value;
#[macro_use]
mod macros;

pub use ast::{ASTKind, AST};
pub use errors::{ReadableConfigError, Result};
pub use lsp_types::Range;
pub use parser::ParserConfig;
pub use value::Value;

/// if ture, { } will be null
pub const BUILD_EMPTY_SCOPE: bool = true;
