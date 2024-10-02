use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::path::Path;
use toml_edit::{value, Document};

#[derive(Debug)]
pub enum ConfigError {
    FileNotFound(String),
    ParseError(String),
    IoError(io::Error),
}

pub struct Config {
    pub document: Document,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::FileNotFound(msg) => write!(f, "File not found: {}", msg),
            ConfigError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ConfigError::IoError(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl Error for ConfigError {}

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> ConfigError {
        if err.kind() == io::ErrorKind::NotFound {
            ConfigError::FileNotFound(err.to_string())
        } else {
            ConfigError::IoError(err)
        }
    }
}

impl From<toml_edit::TomlError> for ConfigError {
    fn from(err: toml_edit::TomlError) -> ConfigError {
        ConfigError::ParseError(err.to_string())
    }
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let config_str = fs::read_to_string(&path)?;
        let document = config_str.parse::<Document>()?;
        Ok(Config { document })
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), ConfigError> {
        fs::write(&path, self.document.to_string())?;
        Ok(())
    }

    pub fn get_base_url(&self) -> Option<&str> {
        self.document["api"]["base"].as_str()
    }

    pub fn set_base_url(&mut self, new_url: &str) {
        self.document["api"]["base"] = value(new_url);
    }

    pub fn get_api_key(&self) -> Option<&str> {
        self.document["api"]["key"].as_str()
    }

    pub fn set_api_key(&mut self, new_key: &str) {
        self.document["api"]["key"] = value(new_key);
    }
}
