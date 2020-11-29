mod wrap_parser;

pub use indexmap::IndexMap;
pub use arc_number::{Number,BigUint,BigDecimal,BigInt};

#[cfg(feature = "json")]
pub use wrap_parser::{parse_json};
#[cfg(feature = "toml")]
pub use wrap_parser::{parse_toml};
#[cfg(feature = "yaml")]
pub use wrap_parser::{parse_yaml};

