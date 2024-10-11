use std::error::Error;
use std::{fmt, io};

#[derive(Debug)]
pub enum ConfigError {
    FileNotFound(String),
    ParseError(String),
    IoError(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::FileNotFound(msg) => write!(f, "File not found: {msg}"),
            ConfigError::ParseError(msg) => write!(f, "Parse error: {msg}"),
            ConfigError::IoError(err) => write!(f, "IO error: {err}"),
        }
    }
}

impl Error for ConfigError {}

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> ConfigError {
        if err.kind() == io::ErrorKind::NotFound {
            ConfigError::FileNotFound(err.to_string())
        } else {
            ConfigError::IoError(err.to_string())
        }
    }
}

impl From<toml_edit::TomlError> for ConfigError {
    fn from(err: toml_edit::TomlError) -> ConfigError {
        ConfigError::ParseError(err.to_string())
    }
}
