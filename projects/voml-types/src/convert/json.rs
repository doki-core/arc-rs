use serde::Serialize;
use serde_json::Value;
pub use serde_json::Value as Json;

use crate::{Von, VonSerializer};

impl From<Json> for Von {
    fn from(json: Json) -> Self {
        match json.serialize(VonSerializer::default()) {
            Ok(o) => o,
            Err(e) => {
                todo!("{e}")
            }
        }
    }
}

#[test]
pub fn test_json() {
    assert_eq!(Von::from(Value::Array(vec![Value::Bool(false)])), true);
}
