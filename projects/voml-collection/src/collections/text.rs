use super::*;
use indexmap::IndexMap;
use std::slice::Iter;

#[derive(Clone, Debug, Hash, Eq)]
pub struct Text {
    pub hint: String,
    pub value: String,
}
