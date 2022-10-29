use crate::{
    parser::von::{NamespaceNode, NumberNode, SpecialNode, StringNode},
    Text,
};

use super::*;

impl ValueNode {
    pub fn into_von(self) -> VonNode {
        match self {
            ValueNode::SpecialNode(v) => v.into_von(),
            ValueNode::NumberNode(v) => v.into_von(),
            ValueNode::StringNode(v) => v.into_von(),
            ValueNode::TableNode(_) => {
                todo!()
            }
        }
    }
}
impl SpecialNode {
    pub fn into_von(self) -> VonNode {
        match self.string.as_str() {
            "true" => VonNode::Boolean(true),
            "false" => VonNode::Boolean(false),
            _ => VonNode::Keyword("default"),
        }
    }
}

impl NumberNode {
    pub fn into_von(self) -> VonNode {
        VonNode::Number(self.into_num())
    }

    pub fn into_num(self) -> Number {
        let value = BigDecimal::from_str(&self.num.string).unwrap_or_default();
        Number { hint: into_hint(self.hint), value }
    }
    pub fn into_identifier(self) -> Identifier {
        Identifier { name: self.num.string, span: self.position }
    }
}

impl StringNode {
    pub fn into_von(self) -> VonNode {
        VonNode::Text(self.into_str())
    }
    pub fn into_str(self) -> Text {
        Text { hint: into_hint(self.hint), value: Default::default() }
    }
}

impl IdentifierNode {
    pub fn into_identifier(self) -> Identifier {
        Identifier { name: self.string, span: self.position }
    }
}

impl NamespaceNode {
    pub fn into_namespace(self) -> Vec<Identifier> {
        self.path.into_iter().map(|v| v.into_identifier()).collect()
    }
}
