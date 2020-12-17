#[derive(Clone, Debug, PartialEq)]
pub enum ArcError {
    ParseError(String),
}

pub type Result<T> = std::result::Result<T, ArcError>;
type PestError = arc_pest::Error<arc_pest::Rule>;

impl From<PestError> for ArcError {
    fn from(e: PestError) -> Self {
        Self::ParseError(format!("{:?}", e))
    }
}