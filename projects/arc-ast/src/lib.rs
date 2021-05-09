#![warn(missing_docs)]

//! missing

mod ast;
mod convert;
mod errors;
mod parser;
mod serder;
mod traits;
pub mod utils;
pub mod value;
#[macro_use]
mod macros;

pub use self::ast::{ASTKind, AST};
pub use self::errors::{ReadableConfigError, Result};
pub use lsp_types::Range;
pub use self::parser::ParserConfig;
pub use self::value::Value;
pub use serde;
pub use self::serder::ReadableConfigSerializer;

/// if ture, { } will be null
pub const BUILD_EMPTY_SCOPE: bool = true;
