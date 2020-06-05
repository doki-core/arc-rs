pub use crate::value::{byte::Byte, decimal::Decimal, dict::Dict, integer::Integer, list::List, string::Text};
use std::fmt::{self, Debug, Formatter};

mod byte;
mod decimal;
mod dict;
mod integer;
mod list;
mod string;

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
            Value::Boolean(v) => Debug::fmt(v, f),
            Value::Integer(v) => Debug::fmt(v, f),
            Value::Decimal(v) => Debug::fmt(v, f),
            Value::String(v) => Debug::fmt(v, f),
            Value::List(v) => Debug::fmt(v, f),
            Value::Dict(v) => Debug::fmt(v, f),
        }
    }
}
