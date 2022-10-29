use super::*;

impl Default for VonNode {
    fn default() -> Self {
        VonNode::Default
    }
}

impl Default for Table {
    fn default() -> Self {
        Self { hint: "".to_string(), list: vec![], dict: Default::default() }
    }
}
