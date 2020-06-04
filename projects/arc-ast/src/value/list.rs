use crate::Value;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct List {
    handler: String,
    value: Vec<Value>,
}
