use super::*;

impl Default for Text {
    fn default() -> Self {
        Self { hint: "".to_string(), value: "".to_string() }
    }
}

impl PartialEq for Text {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}
