pub use crate::value::{byte::Byte, decimal::Decimal, dict::Dict, integer::Integer, list::List, string::Text};

mod byte;
mod decimal;
mod dict;
mod integer;
mod list;
mod string;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Value {
    Null,
    Boolean(bool),
    Integer(Box<Integer>),
    Decimal(Box<Decimal>),
    String(Box<Text>),
    List(Box<List>),
    Dict(Box<Dict>),
}

impl Default for Value {
    fn default() -> Self {
        Self::Null
    }
}

// impl From<()> for Value {
//     fn from(v: ()) -> Self {
//         Self::List(Box::new())
//     }
// }
