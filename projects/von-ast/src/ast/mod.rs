use bigdecimal::BigDecimal;
use diagnostic::Span;
use indexmap::IndexMap;

mod display;
mod ser;
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
    pub hint: Option<Identifier>,
    pub value: String,
}

#[derive(Clone)]
pub struct List {
    pub hint: Option<Identifier>,
    pub value: Vec<VonNode>,
}

#[derive(Clone)]
pub struct Dict {
    pub hint: Option<Identifier>,
    pub value: IndexMap<String, VonNode>,
}

#[derive(Clone)]
pub struct Identifier {
    pub id: String,
    pub span: Span,
}
