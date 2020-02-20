use arc_number::Number;
use linked_hash_map::LinkedHashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Arc {
    Null,
    Boolean(bool),
    Number(Number),
    Char(char),
    String(String),
    Cite(Vec<String>),
    List(Vec<Arc>),
    Dict(LinkedHashMap<String, Arc>),
    Macro(String, String),
}
