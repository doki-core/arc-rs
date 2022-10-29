use super::*;

impl Default for VonNode {
    fn default() -> Self {
        VonNode::Default
    }
}

impl Default for Text {
    fn default() -> Self {
        Self { hint: "".to_string(), value: "".to_string() }
    }
}
