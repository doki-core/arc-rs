use crate::{Arc, Error, Result};
use serde::{ser, Serialize};
use std::fmt::Display;
// impl Serialize for Arc {
// #[inline]
// fn serialize<S>(&self, serializer: S) -> Result<S::Ok)>
// where
// S: ::serde::Serializer,
// {
// match *self {
// Arc::Null => serializer.serialize_unit(),
// Arc::Boolean(b) => serializer.serialize_bool(b),
// Arc::Number(ref n) => n.serialize(serializer),
// Arc::Char(ref c) => c.serialize(serializer),
// Arc::String(ref s) => serializer.serialize_str(s),
// Arc::Cite(ref c) => c.serialize(serializer),
// Arc::List(ref v) => v.serialize(serializer),
// Arc::Dict(ref o) => {
// unimplemented!()
// }
// _ => unimplemented!(),
// }
// return Ok(())
// }
// }
pub struct Serializer {
    output: String,
    with_handler: bool,
    with_type_hint: bool,
}

impl Default for Serializer {
    fn default() -> Self {
        Self { output: String::new(), with_handler: false, with_type_hint: false }
    }
}
