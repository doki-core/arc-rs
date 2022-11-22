use serde_json::Value;
pub use serde_json::Value as Json;

use crate::{Von, VonSerializer};

impl From<Json> for Von {
    fn from(json: Json) -> Self {
        match json.serialize(VonSerializer {}) {}
    }
}
