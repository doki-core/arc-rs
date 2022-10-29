#![forbid(missing_docs)]
#![doc = include_str!("../Readme.md")]
#![allow(clippy::needless_return)]

mod error;
mod for_3rd;

pub use diagnostic::{Diagnostic, DiagnosticLevel, FileID};
pub use error::{Result, VomlError, VomlErrorKind};
