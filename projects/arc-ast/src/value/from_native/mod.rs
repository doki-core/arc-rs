pub mod byte;
pub mod decimal;
pub mod dict;
pub mod integer;
pub mod list;
pub mod string;

pub use byte::Byte;
pub use decimal::Decimal;
pub use dict::Dict;
pub use integer::Integer;
pub use list::List;
pub use string::Text;

use crate::Value;
use bigdecimal::BigDecimal;
use indexmap::IndexMap;
use num::BigInt;
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque},
    convert::TryFrom,
    fmt::{self, Debug, Formatter},
};

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Self::Boolean(v)
    }
}
