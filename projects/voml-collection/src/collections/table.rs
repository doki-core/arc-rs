use super::*;
use indexmap::IndexMap;
use std::slice::Iter;

#[derive(Clone, Debug, Eq)]
pub struct Table<T> {
    pub hint: String,
    pub list: Vec<T>,
    pub dict: IndexMap<String, T>,
}
