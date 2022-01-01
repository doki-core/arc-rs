mod literal_pattern;
mod ordered_map;
mod ordered_set;
mod sparse_array;
mod traits;

use std::collections::BTreeMap;
use indexmap::{IndexMap, IndexSet};
use num::BigUint;
use yggdrasil_shared::records::Literal;
pub use literal_pattern::*;
pub use ordered_map::*;
pub use ordered_set::*;
pub use sparse_array::*;


/// Ordered set of values
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct OrderedSet<T> {
    inner: IndexSet<Literal<T>>,
}

/// Ordered map of key value pairs
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct OrderedMap<T> {
    inner: IndexMap<String, KVPair<T>>,
}

/// Ordered map of key value pairs
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KVPair<T> {
    key: Literal<String>,
    value: Literal<T>,
}

/// Literal Patterns for command
#[derive(Clone, Default, Eq, PartialEq)]
pub struct LiteralVector<T> {
    inner: Vec<Literal<T>>,
}

/// Sparse representation of the array, the subscript can be any non-zero integer
/// 1-index
#[derive(Clone, Default, Debug, Eq, PartialEq, Hash)]
pub struct SparseArray<T> {
    default: T,
    inner: BTreeMap<BigUint, Literal<T>>,
}
