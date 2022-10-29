use bigdecimal::BigDecimal;
use indexmap::IndexMap;

mod collection;
#[cfg(feature = "serde")]
mod der;
pub mod display;
mod number;
#[cfg(feature = "serde")]
mod ser;

mod text;

#[derive(Clone)]
pub enum VonNode {
    Default,
    Boolean(bool),
    Number(Number),
    Text(Text),
    List(List),
    Dict(Dict),
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
pub struct List {
    pub hint: String,
    pub value: Vec<VonNode>,
}

#[derive(Clone)]
pub struct Dict {
    pub hint: String,
    pub value: IndexMap<String, VonNode>,
}
