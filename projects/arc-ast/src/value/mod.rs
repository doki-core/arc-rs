

#[derive(Clone, Eq, PartialEq)]
pub enum Value {

    Null,

    Bool(bool),

    Integer(Number),
    Byte(String),

    String(String),

    List(Vec<Value>),

    Dict(Map<String, Value>),
}