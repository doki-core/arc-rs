use crate::AST;
use std::collections::HashMap;

impl AST {
    pub fn new_string(handler: &str, data: &str) -> AST {
        unimplemented!();
        AST::String(String::new())
    }
    pub fn new_number(handler: &str, data: &str) -> AST {
        unimplemented!();
    }
    pub fn new_dict() -> AST {
        unimplemented!();
        AST::Dict(HashMap::new())
    }
    pub fn new_list() -> AST {
        unimplemented!();
        AST::List(Vec::new())
    }
}

impl AST {
    pub fn is_string(&self) -> bool {
        match *self {
            AST::String(_) => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match *self {
            AST::Number(_) => true,
            _ => false,
        }
    }

    pub fn is_boolean(&self) -> bool {
        match *self {
            AST::Boolean(_) => true,
            _ => false,
        }
    }

    pub fn is_null(&self) -> bool {
        match *self {
            AST::Null => true,
            _ => false,
        }
    }
}
