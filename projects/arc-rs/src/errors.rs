#[derive(Clone, Debug, PartialEq)]
pub enum ArcError {
    Message(String),
}

pub type Result<T> = std::result::Result<T, ArcError>;
