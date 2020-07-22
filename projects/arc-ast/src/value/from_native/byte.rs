use super::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Byte {
    handler: Option<String>,
    value: String
}
