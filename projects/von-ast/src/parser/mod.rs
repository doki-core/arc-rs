use std::str::FromStr;

use bigdecimal::BigDecimal;
use peginator::PegParser;

use voml_error::{FileID, Result, Span, Validation, VomlError};

use crate::{
    parser::von::{IdentifierNode, TableNode, ValueNode, VonParser},
    Number, VonNode,
};

mod number;
mod table;
mod value;
mod von;

pub struct ParserState {
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

fn into_hint(v: Option<IdentifierNode>) -> String {
    match v {
        Some(s) => s.string,
        None => String::new(),
    }
}

impl ParserState {
    pub fn parse_text(text: String, file_id: FileID) -> Result<Self> {
        let mut parser = Self { ast: Default::default(), file_id, text, errors: vec![] };
        parser.do_parse()?;
        Ok(parser)
    }

    fn do_parse(&mut self) -> Result {
        let value = VonParser::parse(&self.text)?.value;
        self.ast = value.into_von(self);
        return Ok(());
    }
}
