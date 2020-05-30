pub use crate::value::integer::{Integer, };
pub use crate::value::string::Text;
pub use crate::value::list::List;
pub use crate::value::dict::Dict;
pub use crate::value::byte::Byte;
pub use crate::value::decimal::Decimal;

mod integer;
mod decimal;
mod list;
mod byte;
mod dict;
mod string;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Value {
    Null,
    Boolean(bool),
    Integer(Box<Integer>),
    Decimal(Box<Decimal>),
    Byte(Box<Byte>),
    String(Box<Text>),
    List(Box<List>),
    Dict(Box<Dict>),
}

