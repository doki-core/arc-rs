use crate::{
    parser::von::{NumberNode, SpecialNode, StringNode},
    Text,
};

use super::*;

impl ValueNode {
    pub fn into_von(self) -> VonNode {
        match self {
            ValueNode::NumberNode(v) => VonNode::Number(v.into_von()),
            ValueNode::SpecialNode(v) => v.into_von(),
            ValueNode::StringNode(v) => v.into_von(),
            ValueNode::TableNode(_) => {
                todo!()
            }
        }
    }
}

impl NumberNode {
    pub fn into_von(self) -> Number {
        let value = BigDecimal::from_str(&self.num.string).unwrap_or_default();
        Number { hint: as_hint(self.hint), value }
    }
}

impl StringNode {
    pub fn into_von(self) -> Text {
        Text { hint: as_hint(self.hint), value: Default::default() }
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
