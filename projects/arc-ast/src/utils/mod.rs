mod wrap_parser;

// pub use arc_number::{parse_decimal, parse_integer, parse_number, BigDecimal, BigInt, BigUint, Number};
pub use indexmap::IndexMap;

#[cfg(feature = "json")]
pub use wrap_parser::parse_json;
#[cfg(feature = "toml")]
pub use wrap_parser::parse_toml;
#[cfg(feature = "yaml")]
pub use wrap_parser::parse_yaml;
