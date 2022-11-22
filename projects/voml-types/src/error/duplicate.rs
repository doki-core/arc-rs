use super::*;
/// dup
#[derive(Debug)]
pub struct DuplicateItem {
    /// kind
    pub kind: &'static str,
    /// item
    pub item: String,
    /// lsh
    pub lhs: Span,
    /// rhs
    pub rhs: Span,
}

impl DuplicateItem {
    /// build error
    pub fn build_error(self, file: FileID) -> VError {
        VError { kind: Box::new(VErrorKind::Duplicate(self)), level: DiagnosticLevel::Error, file }
    }
}
