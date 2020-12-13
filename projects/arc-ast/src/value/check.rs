use super::*;

impl Value {
    pub fn is_null(&self) -> bool {
        match self {
            Value::Null => true,
            _ => false,
        }
    }
    pub fn is_bool(&self) -> bool {
        match self {
            Value::Boolean(_) => true,
            _ => false,
        }
    }
    pub fn is_true(&self) -> bool {
        match self {
            Value::Boolean(true) => true,
            _ => false,
        }
    }
    pub fn is_false(&self) -> bool {
        match self {
            Value::Boolean(false) => true,
            _ => false,
        }
    }
    pub fn is_list(&self) -> bool {
        match self {
            Value::List(_) => true,
            _ => false,
        }
    }
    pub fn is_dict(&self) -> bool {
        match self {
            Value::Dict(_) => true,
            _ => false,
        }
    }
    pub fn is_string(&self) -> bool {
        match self {
            Value::String(_) => true,
            _ => false,
        }
    }
    pub fn is_empty(&self) -> bool {
        match self {
            Value::Dict(_) => true,
            Value::List(_) => true,
            Value::String(_) => true,
            _ => false,
        }
    }
    pub fn is_number(&self) -> bool {
        match self {
            Value::Number(_) => true,
            _ => false,
        }
    }
    pub fn is_integer(&self) -> bool {
        match self {
            Value::Number(n) => n.is_integer(),
            _ => false,
        }
    }
    pub fn is_decimal(&self) -> bool {
        match self {
            Value::Number(n) => n.is_decimal(),
            _ => false,
        }
    }
    pub fn is_zero(&self) -> bool {
        match self {
            Value::Number(n) => n.is_zero(),
            _ => false,
        }
    }
}

impl Value {
    /// Note that a value of null and non-existent key are considered equivalent
    pub fn has_key(&self, key: &str) -> bool {
        match self {
            Value::Dict(dict) => dict.get(key).map(|e| !e.is_null()).unwrap_or_default(),
            _ => false,
        }
    }
    pub fn get_handler(&self) -> Option<String> {
        match self {
            Value::Null | Value::Boolean(_) => None,
            Value::Number(v) => v.get_handler(),
            Value::String(v) => v.get_handler(),
            Value::Byte(v) => v.get_handler(),
            Value::List(v) => v.get_handler(),
            Value::Dict(v) => v.get_handler(),
        }
    }
}
