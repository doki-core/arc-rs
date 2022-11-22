use crate::VError;
use chrono::ParseError;

impl From<ParseError> for VError {
    fn from(e: ParseError) -> Self {
        VError::syntax_error(e.to_string())
    }
}
