use crate::ast::null;
use crate::Arc;
use arc_number::Number;
use std::collections::HashMap;

impl Arc {
    pub fn new() -> Arc {
        Arc::Null
    }
    pub fn new_dict() -> Arc {
        Arc::Dict(HashMap::new())
    }
    pub fn new_list() -> Arc {
        Arc::List(Vec::new())
    }
    pub fn new_boolean(bool: bool) -> Arc {
        Arc::Boolean(bool)
    }
    pub fn new_string(handler: &str, data: &str) -> Arc {
        Arc::String(String::new())
    }
    pub fn new_number(handler: &str, data: &str) -> Arc {
        return match Number::parse(handler, data) {
            Some(n) => Arc::Number(n),
            None => null,
        };
    }
    pub fn new_cite(cite: Vec<String>) -> Arc {
        Arc::Cite(cite)
    }
}

impl Arc {
    pub fn is_string(&self) -> bool {
        match *self {
            Arc::String(_) => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match *self {
            Arc::Number(_) => true,
            _ => false,
        }
    }

    pub fn is_boolean(&self) -> bool {
        match *self {
            Arc::Boolean(_) => true,
            _ => false,
        }
    }

    pub fn is_null(&self) -> bool {
        match *self {
            Arc::Null => true,
            _ => false,
        }
    }
}
