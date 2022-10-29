use super::*;
use crate::parser::von::NamespaceNode;

struct TableState {}

impl TableNode {}

impl KeyNode {
    pub fn as_identifier(&self) -> Identifier {
        match self {
            KeyNode::IdentifierNode(v) => v.as_identifier(),
            KeyNode::NumNode(_) => Identifier::default(),
        }
    }
}

impl NamespaceNode {}
