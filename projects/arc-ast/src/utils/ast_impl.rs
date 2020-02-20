use crate::Arc;
use arc_number::Number;
use linked_hash_map::LinkedHashMap;
use std::collections::VecDeque;

impl Arc {
    pub fn new() -> Arc {
        Arc::Null
    }
    pub fn new_dict() -> Arc {
        Arc::Dict(LinkedHashMap::new())
    }
    pub fn new_list() -> Arc {
        Arc::List(VecDeque::new())
    }
    pub fn new_boolean(bool: bool) -> Arc {
        Arc::Boolean(bool)
    }
    pub fn new_string(_handler: &str, _data: &str) -> Arc {
        Arc::String(String::new())
    }
    pub fn new_number(handler: &str, data: &str) -> Arc {
        return match Number::parse(handler, data) {
            Some(n) => Arc::Number(n),
            None => Arc::Null,
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

pub trait Getter<T> {
    fn get(&mut self, index: T) -> Option<&Arc>;
}

impl Getter<isize> for Arc {
    fn get(&self, index: isize) -> Option<&Arc> {
        match self {
            Arc::List(list) => {
                if index >= 0 {
                    list.get(index as usize)
                }
                else {
                    list.get(list.len() - index as usize)
                }
            }
            _ => None,
        }
    }
}

impl Getter<String> for Arc {
    fn get(&self, index: String) -> Option<&Arc> {
        match self {
            Arc::Dict(dict) => {
                dict.get(&index)
            }
            _ => None,
        }
    }
}

impl Getter<Arc> for Arc {
    fn get(&self, index: Arc) -> Option<&Arc> {
        match index {
            Arc::Index(i) => {
                self.get(i)
            },
            Arc::String(s) => {
                self.get(s)
            },
            _ => None
        }
    }
}

pub trait Setter<T> {
    fn set(&mut self, key: T, value: Arc) -> Option<&Arc>;
}


impl Setter<String> for Arc {
    fn set(&mut self, key: String, value: Arc) -> Option<&Arc> {
        return None;
    }
}

impl Setter<isize> for Arc {
    fn set(&mut self, key: isize, value: Arc) -> Option<&Arc> {
        return None;
    }
}

impl Setter<Vec<Arc>> for Arc {
    fn set(&mut self, key: Vec<Arc>, value: Arc) -> Option<&Arc> {
        let mut pointer = self;
        match self {
            Arc::Dict(a) => {
                for path in key {
                    match path {
                        Arc::Index(i) => match pointer {
                            Arc::Dict(dict) => Some(Arc::Null),
                            _ => None,
                        },
                        Arc::String(s) => None,
                        _ => None,
                    }
                }
            }
            _ => None,
        }
        return None;
    }
}
