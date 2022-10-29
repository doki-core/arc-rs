use crate::{
    parser::von::{SpecialNode, StringNode},
    Text,
};

use super::*;

impl ValueNode {
    pub fn into_von(self) -> VonNode {
        match self {
            ValueNode::NumNode(v) => VonNode::Number(v.into_von()),
            ValueNode::SpecialNode(v) => v.into_von(),
            ValueNode::StringNode(v) => v.into_von(),
            ValueNode::TableNode(_) => {
                todo!()
            }
        }
    }
}

impl NumNode {
    pub fn into_von(self) -> Number {
        Number { hint: "".to_string(), value: Default::default() }
    }
}

impl StringNode {
    pub fn into_von(self) -> Text {
        Text { hint: "".to_string(), value: Default::default() }
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
