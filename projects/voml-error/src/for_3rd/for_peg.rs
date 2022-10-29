use diagnostic::{DiagnosticLevel, Span};
use peginator::ParseError;

use crate::{error::ParseFail, VomlError, VomlErrorKind};

impl From<ParseError> for VomlError {
    fn from(error: ParseError) -> Self {
        let e = ParseFail { message: error.specifics.to_string(), span: Span { start: error.position, end: error.position } };
        VomlError { kind: Box::new(VomlErrorKind::ParseError(e)), level: DiagnosticLevel::Error, file: Default::default() }
    }
}
