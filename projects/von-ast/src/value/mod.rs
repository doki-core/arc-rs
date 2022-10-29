use bigdecimal::BigDecimal;
use indexmap::IndexMap;

#[cfg(feature = "serde")]
mod der;
pub mod display;
mod number;
#[cfg(feature = "serde")]
mod ser;
mod table;

mod text;

#[derive(Clone)]
pub enum VonNode {
    Default,
    Boolean(bool),
    Number(Number),
    Text(Text),
    Table(Table),
}

#[derive(Clone, Hash)]
pub struct Number {
    pub hint: String,
    pub value: BigDecimal,
}

#[derive(Clone, Hash)]
pub struct Text {
    pub hint: String,
    pub value: String,
}

#[derive(Clone)]
pub struct Table {
    pub hint: String,
    pub list: Vec<VonNode>,
    pub dict: IndexMap<String, VonNode>,
}
