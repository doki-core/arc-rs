use crate::VError;
use globset::Error;

impl From<Error> for VError {
    fn from(e: Error) -> Self {
        VError::runtime_error(e.to_string())
    }
}
