use diagnostic::{DiagnosticLevel, Span};
use peginator::ParseError;

use crate::{error::ParseFail, VError, VErrorKind};

impl From<ParseError> for VError {
    fn from(error: ParseError) -> Self {
        let e = ParseFail { message: error.specifics.to_string(), span: Span { start: error.position, end: error.position } };
        VError { kind: Box::new(VErrorKind::ParseError(e)), level: DiagnosticLevel::Error, file: Default::default() }
    }
}
