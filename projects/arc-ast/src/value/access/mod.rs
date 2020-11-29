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
        unimplemented!()
        // let tokens = path.split('.');
        // let mut target = self;
        //
        // for token in tokens {
        //     let target_opt = match *target {
        //         Value::Dict(ref dict) =>dict.get(&token),
        //         Value::List(ref list) => list.get(&tokens),
        //         _ => return None,
        //     };
        //     match target_opt {
        //         Some(t) => {
        //             target = t;
        //         }
        //         None => {
        //             return None;
        //         }
        //     }
        // }
        // Some(target)
    }
    pub fn pointer_mut(&mut self, path: &str) -> Option<&mut Value> {
        if path.is_empty() {
            return Some(self);
        }
        unimplemented!()
    }
}

// #[test]
// fn test() {
//     use serde_json::json;
//     let data = json!({
//     "x": {
//         "y": ["z", "zz"]
//     }
// });
//     let v = Value::from(data).pointer("x.y.1");
//     println!("{:?}",v)
// }
