#[cfg(feature = "rust_decimal")]
pub use decimal::Decimal;

pub use self::{bytes::Bytes, integer::Integer, namespace::Namespace, text::Text};

#[cfg(feature = "rust_decimal")]
mod decimal;

mod integer;
mod namespace;

mod bytes;
// mod number;
mod text;
