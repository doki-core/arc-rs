//! utils


#[allow(dead_code)]
mod wrap_parser;

pub use bigdecimal::BigDecimal;
pub use indexmap::IndexMap;
pub use num::{BigInt, BigUint};

use crate::ast::ExtendFormat;

pub use wrap_parser::parse_arc;
#[cfg(feature = "json")]
pub use wrap_parser::parse_json;
#[cfg(feature = "toml")]
pub use wrap_parser::parse_toml;
#[cfg(feature = "yaml")]
pub use wrap_parser::parse_yaml;

/// if ture, { } will be null
pub const BUILD_EMPTY_SCOPE: bool = false;

pub fn parse_format(extension: &str) -> ExtendFormat {
    match extension.to_ascii_lowercase().as_str() {
        #[cfg(feature = "json")]
        "json" => ExtendFormat::JSON,
        #[cfg(feature = "hjson")]
        "hjson" => ExtendFormat::HJSON,
        #[cfg(feature = "toml")]
        "toml" => ExtendFormat::TOML,
        #[cfg(feature = "yaml")]
        "yaml" => ExtendFormat::YAML,
        "arc" => ExtendFormat::ARC,
        _ => ExtendFormat::TEXT,
    }
}
