use arc_number::Number;
use linked_hash_map::LinkedHashMap;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub enum Arc {
    Null,
    Boolean(bool),
    Index(isize),
    Number(Number),
    Char(char),
    String(String),
    Cite(Vec<String>),
    List(VecDeque<Arc>),
    Dict(LinkedHashMap<String, Arc>),
    Macro(String, String),
}
