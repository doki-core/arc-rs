use diagnostic_quick::{DiagnosticLevel, Failure, FileID, QError, QResult, Success, Validation};
use peginator::PegParser;

use crate::{parser::voml::VomlParser, VEditor, VModel};

#[allow(non_camel_case_types)]
mod voml;

struct ParseContext {
    model: VModel,
    file: FileID,
    errors: Vec<QError>,
}

impl VEditor {
    pub fn parse(&self, text: &str, file: &FileID) -> Validation<VModel> {
        let parser = ParseContext { model: VModel {}, file: file.clone(), errors: vec![] };
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
