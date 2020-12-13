use super::*;

impl Value {
    /// Merge in a configuration property source.
    pub fn merge(&mut self, incoming: Value) {
        if !self.is_dict() || !incoming.is_dict() {
            return *self = incoming;
        }
        match (self, incoming) {
            // FIXME: remove ref in rhs
            (Value::Dict(lhs), Value::Dict(ref rhs)) => {
                for (key, value) in rhs.iter() {
                    let mergeable = match lhs.get(&key) {
                        None => false,
                        Some(s) => value.is_dict() && s.is_dict(),
                    };
                    match mergeable {
                        true => {
                            lhs.get_mut(&key).map(|e| e.merge(value.clone()));
                        }
                        false => {
                            lhs.insert(key.to_string(), value.clone());
                        }
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn pointer(&self, path: &str) -> Option<&Value> {
        if path.is_empty() {
            return Some(self);
        }
        let mut target = self;
        for token in path.split('.') {
            let try_get = match target {
                Value::Dict(dict) => dict.get(&token),
                Value::List(list) => list.get(&token),
                _ => return None,
            };
            if let None = try_get.map(|t| target = t) {
                return None;
            }
        }
        Some(target)
    }
    pub fn pointer_mut(&mut self, path: &str) -> Option<&mut Value> {
        if path.is_empty() {
            return Some(self);
        }
        unimplemented!()
    }

    pub fn as_vec(&self) -> Vec<Value> {
        match self {
            Value::Null => vec![],
            Value::Boolean(_) => vec![self.to_owned()],
            Value::Number(_) => vec![self.to_owned()],
            Value::String(_) => vec![self.to_owned()],
            Value::Byte(_) => vec![self.to_owned()],
            Value::List(v) => v.as_vec(),
            Value::Dict(v) => v.as_vec(),
        }
    }

    pub fn as_string_vec(&self) -> Vec<String> {
        match self {
            Value::Null => vec![],
            Value::Boolean(v) => vec![format!("{}", v)],
            Value::Number(v) => vec![format!("{}", v)],
            Value::String(v) => vec![format!("{:?}", v)],
            Value::Byte(v) => vec![format!("{:?}", v)],
            Value::List(v) => {
                let mut vec = Vec::with_capacity(v.length());
                for item in v.iter() {
                    vec.push(format!("{:?}", item))
                }
                return vec;
            }
            Value::Dict(v) => vec![format!("{:?}", v)],
        }
    }
}

#[test]
fn test() {
    use crate::{dict, list};
    let data = dict! {
        "x": dict!{
            "y": list!["z", "zz"]
        }
    };
    let v = data.pointer("x.y.-1");
    println!("{:?}", v)
}
