mod access;
mod check;
mod decimal;
mod dict;
mod from_native;
mod integer;
mod into_ast;
mod into_native;
mod list;
mod string;

pub use decimal::Decimal;
pub use dict::Dict;
pub use from_native::parse_number;
pub use integer::Integer;
pub use list::List;
pub use string::{Text, TextDelimiter};

use bigdecimal::BigDecimal;
use indexmap::IndexMap;
use num::{BigInt, BigUint};
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque},
    convert::TryFrom,
    fmt::{self, Debug, Display, Formatter},
    ops::Deref,
    str::FromStr,
};

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
