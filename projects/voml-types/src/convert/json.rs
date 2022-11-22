use serde_json::Value;
pub use serde_json::Value as Json;

use crate::Von;

impl From<Json> for Von {
    fn from(json: Json) -> Self {
        match json {
            Value::Null => {
                todo!()
            }
            Value::Bool(v) => {}
            Value::Number(_) => {
                todo!()
            }
            Value::String(_) => {
                todo!()
            }
            Value::Array(_) => {
                todo!()
            }
            Value::Object(_) => {
                todo!()
            }
        }
    }
}
