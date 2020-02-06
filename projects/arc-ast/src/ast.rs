use std::collections::HashMap;
use arc_number::Number;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AST {
    Null,
    Boolean(bool),
    Number(Number),
    String(String),
    Cite(Vec<String>),
    List(Vec<AST>),
    Dict(HashMap<String, AST>),
    Macro,
}

#[allow(non_upper_case_globals)]
pub const null: AST = AST::Null;
