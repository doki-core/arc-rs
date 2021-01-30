pub mod decimal;
pub mod dict;
pub mod integer;
pub mod list;
pub mod string;

pub use decimal::Decimal;
pub use dict::Dict;
pub use integer::Integer;
pub use list::List;
pub use string::{Text, TextDelimiter};

use crate::Value;
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

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Self::List(Box::new(List::default()))
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Self::Boolean(v)
    }
}

impl<T> From<Option<T>> for Value
where
    T: Into<Value>,
{
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => value.into(),
            None => Value::Null,
        }
    }
}

impl<T, E> From<Result<T, E>> for Value
where
    T: Into<Value>,
{
    fn from(value: Result<T, E>) -> Self {
        match value {
            Ok(value) => value.into(),
            Err(_) => Value::Null,
        }
    }
}

