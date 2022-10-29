use bigdecimal::BigDecimal;
use diagnostic::Span;
use indexmap::IndexMap;
use num::BigInt;

mod display;
mod ser;
mod der;

#[derive(Clone)]
pub struct VonNode {
    pub kind: VonKind,
    pub hint: Identifier,
    pub span: Span,
}

#[derive(Clone)]
pub enum VonKind {
    Boolean(bool),
    Integer(BigInt),
    Decimal(BigDecimal),
    Text(String),
    List(Vec<VonNode>),
    Dict(IndexMap<String, VonNode>),
}

#[derive(Clone)]
pub struct Identifier {
    pub id: String,
    pub span: Span,
}
