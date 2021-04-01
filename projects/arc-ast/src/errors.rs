use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

/// result type
pub type Result<T> = std::result::Result<T, ReadableConfigError>;

/// error type
#[derive(Debug)]
pub enum ReadableConfigError {
    /// missing
    IOError(String),
    /// missing
    LexerError(String),
    /// missing
    OtherError(String),
    /// missing
    CustomError(Box<dyn Error>),
}

type IOError = std::io::Error;
type JsonError = serde_json::Error;
type TomlError = toml::de::Error;
type YamlError = yaml_rust::ScanError;

impl Error for ReadableConfigError {}

// impl<S: ToString> From<S> for ReadableConfigError {
//     fn from(other: S) -> Self {
//         Self::OtherError(other.to_string())
//     }
// }
//
// impl<S: Error> From<S> for ReadableConfigError {
//     fn from(other: S) -> Self {
//         Self::CustomError(Box::new(other))
//     }
// }

impl Display for ReadableConfigError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl From<IOError> for ReadableConfigError {
    fn from(e: IOError) -> Self {
        Self::IOError(format!("{}", e))
    }
}

impl From<TomlError> for ReadableConfigError {
    fn from(e: TomlError) -> Self {
        Self::LexerError(format!("{}", e))
    }
}

impl From<JsonError> for ReadableConfigError {
    fn from(e: JsonError) -> Self {
        Self::LexerError(format!("{}", e))
    }
}

impl From<YamlError> for ReadableConfigError {
    fn from(e: YamlError) -> Self {
        Self::LexerError(format!("{}", e))
    }
}

impl ReadableConfigError {
    /// custom error
    pub fn custom(e: impl Error + 'static) -> Self {
        Self::CustomError(Box::new(e))
    }
    /// other error
    pub fn other(msg: impl ToString) -> Self {
        Self::OtherError(msg.to_string())
    }
}
