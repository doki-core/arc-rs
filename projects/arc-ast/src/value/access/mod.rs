use super::*;

impl Value {
    /// Merge in a configuration property source.
    pub fn merge(&mut self, incoming: Value) -> ()
    {
        match (self, incoming) {
            // FIXME: Remove ref in rhs side
            (Value::Dict(lhs), Value::Dict(ref rhs)) => {
                for (key, value) in rhs.iter()  {
                    match lhs.get(&key).is_dict() && value.is_dict() {
                        true => {
                            lhs.get_mut(&key).map(|e|e.merge(value.clone()));
                        }
                        false => {
                            lhs.insert(key.to_string(), value.clone());
                        }
                    }
                }
            }
            _ => unreachable!()
        }
    }
    pub fn pointer(&self, path: &str) {
        unimplemented!()
    }
    pub fn pointer_mut(&mut self, path: &str) {
        unimplemented!()
    }
}
