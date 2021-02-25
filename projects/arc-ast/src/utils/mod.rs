mod wrap_parser;

pub use bigdecimal::BigDecimal;
pub use indexmap::IndexMap;
pub use num::{BigInt, BigUint};

#[cfg(feature = "json")]
pub use wrap_parser::parse_json;
#[cfg(feature = "toml")]
pub use wrap_parser::parse_toml;
#[cfg(feature = "yaml")]
pub use wrap_parser::parse_yaml;


pub const BUILD_EMPTY_SCOPE: bool = false;