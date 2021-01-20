mod access;
mod check;
mod from_native;
mod into_ast;
mod into_native;

pub use crate::value::from_native::{Dict, List, Text, TextDelimiter};
use std::fmt::{self, Debug, Display, Formatter};
pub use crate::value::from_native::integer::Integer;
pub use crate::value::from_native::decimal::Decimal;

#[derive(Clone, Eq, PartialEq)]
pub enum Value {
    Null,
    Boolean(bool),
    Integer(Box<Integer>),
    Decimal(Box<Decimal>),
    String(Box<Text>),
    List(Box<List>),
    Dict(Box<Dict>),
}

impl Default for Value {
    fn default() -> Self {
        Self::Null
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Boolean(v) => Display::fmt(v, f),
            Value::Integer(v) => Display::fmt(v, f),
            Value::Decimal(v) => Display::fmt(v, f),
            Value::String(v) => Display::fmt(v, f),
            Value::List(v) => Debug::fmt(v, f),
            Value::Dict(v) => Debug::fmt(v, f),
        }
    }
}
