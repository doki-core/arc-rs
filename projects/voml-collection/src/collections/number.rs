use super::*;
use std::slice::Iter;

#[derive(Clone, Debug, Hash, Eq)]
pub struct Number {
    pub hint: String,
    pub value: BigDecimal,
}
