use crate::Arc;
use arc_number::Number;
use linked_hash_map::LinkedHashMap;

impl Arc {
    pub fn new() -> Arc {
        Arc::Null
    }
    pub fn new_dict() -> Arc {
        Arc::Dict(LinkedHashMap::new())
    }
    pub fn new_list() -> Arc {
        Arc::List(Vec::new())
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

impl Arc {
    pub fn get<N>(&self, index: N) -> Option<&Arc>
    where
        i64: From<N>,
    {
        let i = i64::from(index);
        match self {
            Arc::List(l) => {
                if i == 0 {
                    None
                }
                else if i > 0 {
                    l.get(i as usize)
                }
                else {
                    l.get(l.len() - i as usize)
                }
            }
            _ => None,
        }
    }
    pub fn get_value(&self) {}
    pub fn get_key_value(&self) {}
}
