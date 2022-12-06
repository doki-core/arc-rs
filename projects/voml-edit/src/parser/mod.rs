use diagnostic_quick::{DiagnosticLevel, Failure, FileID, QError, QResult, Success, Validation};
use peginator::PegParser;

use crate::{parser::voml::VomlParser, VomlEditor, VomlModel};

#[allow(non_camel_case_types)]
mod voml;

struct ParseContext {
    model: VomlModel,
    file: FileID,
    errors: Vec<QError>,
}

impl VomlEditor {
    pub fn parse(&self, text: &str, file: &FileID) -> Validation<VomlModel> {
        let parser = ParseContext { model: VomlModel {}, file: file.clone(), errors: vec![] };
        match parser.parse(text) {
            Ok(_) => Success { value: parser.model, diagnostics: parser.errors },
            Err(e) => Failure { fatal: e.with_file(file).with_level(DiagnosticLevel::Fatal), diagnostics: parser.errors },
        }
    }
}

impl ParseContext {
    fn parse(&self, text: &str) -> QResult {
        let parsed = VomlParser::parse(text)?;
        println!("Parsed: {:?}", parsed);
        Ok(())
    }
}
