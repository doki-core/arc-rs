use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

pub type Result<T> = std::result::Result<T, RuntimeError>;

#[derive(Debug)]
pub enum RuntimeError {
    IOError(String),
    LexerError(String),
    OtherError(Box<dyn Error>)
}

type IOError = std::io::Error;
type JsonError = serde_json::Error;
type TomlError = toml::de::Error;
type YamlError = yaml_rust::ScanError;

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for RuntimeError {}

impl From<IOError> for RuntimeError {
    fn from(e: IOError) -> Self {
        Self::IOError(format!("{}", e))
    }
}

impl From<TomlError> for RuntimeError {
    fn from(e: TomlError) -> Self {
        Self::LexerError(format!("{}", e))
    }
}

impl From<JsonError> for RuntimeError {
    fn from(e: JsonError) -> Self {
        Self::LexerError(format!("{}", e))
    }
}

impl From<YamlError> for RuntimeError {
    fn from(e: YamlError) -> Self {
        Self::LexerError(format!("{}", e))
    }
}
