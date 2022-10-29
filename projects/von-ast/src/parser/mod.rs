use std::str::FromStr;

use bigdecimal::BigDecimal;
use diagnostic::Span;
use peginator::PegParser;

use voml_error::{FileID, Result, Validation, VomlError};

use crate::{
    parser::von::{KeyNode, NumNode, TableNode, ValueNode, VonParser},
    Number, VonNode,
};
use crate::parser::von::IdentifierNode;

mod number;
mod table;
mod value;
mod von;

struct ParserState {
    ast: VonNode,
    file_id: FileID,
    text: String,
    errors: Vec<VomlError>,
}

pub struct Identifier {
    name: String,
    span: Span,
}


pub fn parse(text: &str, id: &FileID) -> Validation<VonNode> {
    match ParserState::parse_text(text.to_string(), id.clone()) {
        Ok(o) => Validation::Success { value: o.ast, diagnostics: o.errors },
        Err(e) => Validation::Failure { fatal: e, diagnostics: vec![] },
    }
}

fn as_hint(v: Option<IdentifierNode>) -> String {
    match v {
        Some(s) => s.string,
        None => String::new(),
    }
}

fn as_value(v: &Option<ValueNode>) -> Result<VonNode> {
    match v {
        Some(s) => s.as_value(),
        None => Ok(VonNode::default()),
    }
}

impl ParserState {
    pub fn parse_text(text: String, file_id: FileID) -> Result<Self> {
        let mut parser = Self { ast: Default::default(), file_id, text, errors: vec![] };
        parser.do_parse()?;
        Ok(parser)
    }

    fn do_parse(&mut self) -> Result {
        match VonParser::parse(&self.text)?.value {}
        return Ok(());
    }
}
