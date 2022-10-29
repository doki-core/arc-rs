use bigdecimal::BigDecimal;
use diagnostic::Span;
use indexmap::IndexMap;

mod display;
#[cfg(feature = "serde")]
mod ser;
#[cfg(feature = "serde")]
mod der;
mod number;

#[derive(Clone)]
pub enum VonNode {
    Boolean(bool),
    Number(Number),
    Text(Text),
    List(List),
    Dict(Dict),
}

#[derive(Clone, Hash)]
pub struct Number {
    pub hint: Option<Identifier>,
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

#[derive(Clone)]
pub struct Identifier {
    pub id: String,
    pub span: Span,
}
