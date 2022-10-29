use super::*;

impl Default for Table {
    fn default() -> Self {
        Self { hint: "".to_string(), list: vec![] }
    }
}

impl Default for Table {
    fn default() -> Self {
        Self { hint: "".to_string(), dict: Default::default() }
    }
}
