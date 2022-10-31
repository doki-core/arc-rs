use crate::{Number, Table, Text};
use std::hash::Hash;

#[cfg(feature = "serde")]
mod der;
pub mod display;

#[cfg(feature = "serde")]
mod ser;
mod table;

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum VonNode {
    Keyword(&'static str),
    Boolean(bool),
    Number(Number),
    Text(Text),
    Table(Table),
}
