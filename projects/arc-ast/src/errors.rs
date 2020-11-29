#[derive(Debug, Clone)]
pub enum RuntimeError {
    IOError(String),
    FormatError(String),
}

pub type Result<T> = std::result::Result<T, RuntimeError>;
type IOError = std::io::Error;
type JsonError = serde_json::Error;
type TomlError = toml::de::Error;
type YamlError = serde_json::Error;

impl From<IOError> for RuntimeError {
    fn from(e: IOError) -> Self {
        Self::IOError(format!("{}",e))
    }
}



impl From<TomlError> for RuntimeError {
    fn from(e: TomlError) -> Self {
        Self::FormatError(format!("{}",e))
    }
}

impl From<JsonError> for RuntimeError {
    fn from(e: JsonError) -> Self {
        Self::FormatError(format!("{}",e))
    }
}