
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Text {
    pub hint: String,
    pub value: String,
}

impl Text {
    pub fn new(text: impl Into<String>, hint: impl Into<String>) -> Self {
        Self { hint: hint.into(), value: text.into() }
    }
}
