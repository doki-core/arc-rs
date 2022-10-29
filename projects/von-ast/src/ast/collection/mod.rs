use super::*;

impl Default for List {
    fn default() -> Self {
        Self { hint: "".to_string(), value: vec![] }
    }
}

impl Default for Dict {
    fn default() -> Self {
        Self { hint: "".to_string(), value: Default::default() }
    }
}
