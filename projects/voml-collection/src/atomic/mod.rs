#[cfg(feature = "rust_decimal")]
pub use self::decimal::Decimal;
#[cfg(feature = "bigdecimal")]
pub use self::number::Number;
pub use self::{bytes::Bytes, integer::Integer, namespace::Namespace, text::Text};

#[cfg(feature = "rust_decimal")]
mod decimal;
mod integer;
mod namespace;

mod bytes;
#[cfg(feature = "bigdecimal")]
mod number;
mod text;
