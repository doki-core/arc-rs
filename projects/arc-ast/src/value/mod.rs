mod access;
mod check;
mod from_native;
mod into_ast;
mod into_native;

pub use crate::value::from_native::{Dict, List, Text, TextDelimiter};
pub use arc_number::Number;
use std::fmt::{self, Debug, Display, Formatter};

#[derive(Clone, Eq, PartialEq)]
pub enum Value {
    Null,
    Boolean(bool),
    Number(Box<Number>),
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
            Value::Number(v) => Display::fmt(v, f),
            Value::String(v) => Display::fmt(v, f),
            Value::List(v) => Debug::fmt(v, f),
            Value::Dict(v) => Debug::fmt(v, f),
        }
    }
}
