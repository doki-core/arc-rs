pub mod ast_from;
pub mod ast_impl;
pub mod ast_traits;

use arc_number::Number;
use linked_hash_map::LinkedHashMap;
use std::collections::VecDeque;

#[derive(Clone, PartialEq)]
pub enum Arc {
    Null,
    Boolean(bool),
    Number(Number),
    Char(char),
    String(String),
    Cite(Vec<String>),
    List(VecDeque<Arc>),
    Dict(LinkedHashMap<String, Arc>),
    Macro(String, String),
}
