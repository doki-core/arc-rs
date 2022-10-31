// mod literal_pattern;
// mod ordered_map;
// mod ordered_set;
// mod sparse_array;
// mod traits;

pub use self::{decimal::Decimal, dict::Dict, integer::Integer, list::List, number::Number, table::Table, text::Text};

mod decimal;
mod dict;
mod integer;
mod list;
mod number;
mod table;
mod text;

// use indexmap::{IndexMap, IndexSet};
// pub use literal_pattern::*;
// use num::BigUint;
// pub use ordered_map::*;
// pub use ordered_set::*;
// pub use sparse_array::*;
// use std::collections::BTreeMap;
// use yggdrasil_shared::records::Literal;
//
// /// Ordered set of values
// #[derive(Clone, Default, Debug)]
// pub struct OrderedSet<T> {
//     inner: IndexSet<Literal<T>>,
// }
//
// /// Ordered map of key value pairs
// #[derive(Clone, Default, Debug)]
// pub struct OrderedMap<T> {
//     inner: IndexMap<String, KVPair<T>>,
// }
//
// /// Ordered map of key value pairs
// #[derive(Clone, Debug)]
// pub struct KVPair<T> {
//     key: Literal<String>,
//     value: Literal<T>,
// }
//
// /// Literal Patterns for command
// #[derive(Clone, Default, Hash)]
// pub struct LiteralVector<T> {
//     inner: Vec<Literal<T>>,
// }
//
// /// Sparse representation of the array, the subscript can be any non-zero integer
// /// 1-index
// #[derive(Clone, Default, Debug, Hash)]
// pub struct SparseArray<T> {
//     default: T,
//     inner: BTreeMap<BigUint, Literal<T>>,
// }
