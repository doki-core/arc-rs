#![forbid(missing_docs)]
#![doc = include_str!("../Readme.md")]

pub use diagnostic::{Diagnostic, DiagnosticLevel, FileID};

pub use self::error::{Result, Validation, VomlError, VomlErrorKind};

mod error;
mod for_3rd;
