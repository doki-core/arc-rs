use indexmap::IndexMap;

use voml_error::DuplicateItem;

use crate::{
    parser::von::{KeyNode, TableItem},
    Table,
};

use super::*;

struct TableBuilder {
    hint: String,
    list: Vec<VonNode>,
    dict: IndexMap<String, (VonNode, Span)>,
}

impl TableNode {
    pub fn into_von(self, ctx: &mut ParserState) -> VonNode {
        let mut builder = TableBuilder { hint: into_hint(self.hint), list: vec![], dict: Default::default() };

        for item in self.items {
            match item {
                TableItem::KeyValueNode(pair) => {
                    let Identifier { name, span } = pair.key.into_identifier();
                    let value = pair.value.into_von(ctx);
                    if let Some(s) = builder.dict.insert(name.clone(), (value, span.clone())) {
                        let e = DuplicateItem { kind: "field", item: name, lhs: s.1, rhs: span };
                        ctx.errors.push(e.build_error(ctx.file_id.clone()))
                    }
                }
                TableItem::ValueNode(v) => builder.list.push(v.into_von(ctx)),
                TableItem::Split(_) => continue,
            }
        }
        VonNode::Table(builder.build())
    }
}

impl TableBuilder {
    pub fn build(self) -> Table {
        Table { hint: self.hint, list: self.list, dict: Default::default() }
    }
}

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
