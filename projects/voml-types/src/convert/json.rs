use serde::Serialize;
pub use serde_json::Value as Json;

use crate::{Serializer, Von};

impl From<Json> for Von {
    fn from(json: Json) -> Self {
        match json.serialize(Serializer::default()) {
            Ok(o) => o,
            Err(e) => {
                todo!("{e}")
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use serde_json::Value;

    use super::*;

    #[test]
    pub fn test_json() {
        assert_eq!(Von::from(Value::Array(vec![Value::Bool(false)])), true);
    }

    #[test]
    fn test_map() {
        let mut map = BTreeMap::new();
        map.insert(Some("0"), true);
        map.insert(Some("1"), false);

        println!("{:#?}", serde_json::to_string(&map).unwrap());
    }
}
