use super::*;

impl Value {
    /// Merge in a configuration property source.
    pub fn merge<T>(&mut self, incoming: IndexMap<String, Value>) -> &mut Value
    {
        let dict = match self {
            Value::Dict(v) => {v},
            _ => unreachable!()
        };
        for (key, value) in incoming.into_iter() {
            dict.insert(key, value);
        }
        unimplemented!()
    }
}
