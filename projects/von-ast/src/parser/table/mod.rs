use super::*;
use crate::parser::von::KeyNode;
struct TableState {}

impl TableNode {}

impl KeyNode {
    pub fn into_identifier(self) -> Identifier {
        match self {
            KeyNode::IdentifierNode(v) => v.into_identifier(),
            KeyNode::StringNode(_) => {
                todo!()
            }
        }
    }
}
