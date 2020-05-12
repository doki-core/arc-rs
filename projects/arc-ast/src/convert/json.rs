use crate::Value;

type Json = serde_json::Value;


impl  From<Json> for Value {
    fn from(json: Json) -> Self {
       match json {
           Json::Null => Self::Null,
           Json::Bool(_) => {}
           Json::Number(_) => {}
           Json::String(_) => {}
           Json::Array(_) => {}
           Json::Object(_) => {}
       }
    }
}

