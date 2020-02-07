use arc_number::Number;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Arc {
    Null,
    Boolean(bool),
    Number(Number),
    String(String),
    Cite(Vec<String>),
    List(Vec<Arc>),
    Dict(HashMap<String, Arc>),
    Macro(String, String),
}

#[allow(non_upper_case_globals)]
pub const null: Arc = Arc::Null;
