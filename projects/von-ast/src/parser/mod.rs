use crate::{
    parser::von::{NumNode, ValueNode},
    Number, VonNode,
};
use bigdecimal::BigDecimal;
use diagnostic::{FileID, Validation};
use peginator::PegParser;
use std::{cmp::Ordering, ops::Range, str::FromStr};
use voml_error::Result;

mod field;
mod number;
mod symbol;
mod value;
mod von;

struct ParserState {
    ast: VonNode,
    file_id: FileID,
    text: String,
    errors: Vec<VomlError>,
}

pub fn parse(text: &str, id: &FileID) -> Validation<VonNode> {
    match ParserState::parse_text(text.to_string(), id.clone()) {
        Ok(o) => Validation::Success { value: o.ast, diagnostics: o.errors },
        Err(e) => Validation::Failure { fatal: e, diagnostics: vec![] },
    }
}

fn as_value(v: &Option<ValueNode>) -> Result<ValueStatement> {
    match v {
        Some(s) => s.as_value(),
        None => Ok(ValueStatement::default()),
    }
}

impl ParserState {
    pub fn parse_text(text: String, file_id: FileID) -> Result<Self> {
        let mut parser = Self { ast: Default::default(), file_id, text, errors: vec![] };
        parser.do_parse()?;
        Ok(parser)
    }

    fn do_parse(&mut self) -> Result {
        for statement in VonParser::parse(&self.text)?.statements {
            match self.visit_statement(statement) {
                Ok(_) => {}
                Err(e) => self.errors.push(e),
            }
        }
        return Ok(());
    }
    fn visit_statement(&mut self, node: VonParser) -> Result {
        match node {
            VosStatementNode::StructDeclareNode(s) => {
                let mut table = TableStatement::default();
                table.kind = TableKind::Structure;
                self.push_table(table, s.id, s.body)?
            }
            VosStatementNode::TableDeclareNode(s) => {
                let mut table = TableStatement::default();
                table.kind = TableKind::Table;
                self.push_table(table, s.id, s.body)?
            }
            VosStatementNode::ObjectStatementNode(s) => self.ast.push_object(s.id.as_identifier(), s.value.as_value()?),
            VosStatementNode::UnionStatementNode(_) => {
                todo!()
                // s.id.as_identifier()
                // s.body
            }
            VosStatementNode::Split(_) => {}
        }
        Ok(())
    }
    fn push_table(&mut self, mut table: TableStatement, id: IdentifierNode, body: Vec<DeclareBodyNode>) -> Result {
        table.name = id.as_identifier();
        for term in body {
            match term {
                DeclareBodyNode::FieldStatementNode(v) => match table.add_field(v.as_field()?) {
                    Ok(_) => {}
                    Err(e) => {
                        let error = DuplicateDeclare {
                            kind: "field",
                            symbol: e.name.id.to_string(),
                            lhs: e.name.span,
                            rhs: v.key.as_identifier().span,
                        };
                        self.errors.push(error.build(self.file_id.clone()))
                    }
                },
                DeclareBodyNode::ConstraintStatementNode(v) => table.add_constraint(v.as_constraint()?),
                DeclareBodyNode::Split(_) => {}
            }
        }
        self.ast.statements.push(VosStatement::Table(Box::new(table)));
        Ok(())
    }
}

impl KeyNode {
    pub fn as_identifier(&self) -> Identifier {
        match self {
            KeyNode::IdentifierNode(v) => v.as_identifier(),
            KeyNode::NumNode(_) => Identifier::default(),
        }
    }
}
