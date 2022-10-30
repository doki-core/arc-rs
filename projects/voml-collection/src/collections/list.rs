use super::*;
use indexmap::IndexMap;
use std::slice::Iter;

#[derive(Clone, Debug, Eq)]
pub struct List<T> {
    pub hint: String,
    pub list: Vec<T>,
}
