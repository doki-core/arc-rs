use serde::{Deserialize, Serialize};

/// A text with the format
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bytes {
    /// The format of this text
    pub hint: String,
    /// The content of this text
    pub text: Vec<u8>,
}

impl Bytes {
    /// Create a new text
    pub fn new(text: Vec<u8>, hint: impl Into<String>) -> Self {
        Self { hint: hint.into(), text: text.into() }
    }
}

impl From<&[u8]> for Bytes {
    fn from(value: &[u8]) -> Self {
        Self { hint: "".to_string(), text: value.to_vec() }
    }
}
