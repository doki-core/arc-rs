use bigdecimal::BigDecimal;
use indexmap::IndexMap;
use std::hash::{Hash, Hasher};
#[cfg(feature = "serde")]
mod der;
pub mod display;
mod number;
#[cfg(feature = "serde")]
mod ser;
mod table;

mod text;

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum VonNode {
    Keyword(&'static str),
    Boolean(bool),
    Number(Number),
    Text(Text),
    Table(Table),
}

#[derive(Clone,Debug, Hash, Eq)]
pub struct Number {
    pub hint: String,
    pub value: BigDecimal,
}

#[derive(Clone, Debug, Hash, Eq)]
pub struct Text {
    pub hint: String,
    pub value: String,
}

#[derive(Clone, Debug, Eq)]
pub struct Table {
    pub hint: String,
    pub list: Vec<VonNode>,
    pub dict: IndexMap<String, VonNode>,
}
