use crate::parser::von::SpecialNode;

use super::*;

impl NumNode {
    pub fn into_von(self) -> Number {
        Number { hint: "".to_string(), value: Default::default() }
    }
}

impl SpecialNode {
    pub fn into_von(self) -> VonNode {
        match self.string.as_str() {
            "true" => VonNode::Boolean(true),
            "false" => VonNode::Boolean(false),
            _ => VonNode::Default,
        }
    }
}
