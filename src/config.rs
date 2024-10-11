use dotenv::dotenv;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::path::Path;
use toml_edit::{value, DocumentMut, Item};

#[derive(Debug)]
pub enum ConfigError {
    FileNotFound(String),
    ParseError(String),
    IoError(String),
}

pub struct Config {
    document: DocumentMut,
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
            ConfigError::IoError(err.to_string())
        }
    }
}

impl From<toml_edit::TomlError> for ConfigError {
    fn from(err: toml_edit::TomlError) -> ConfigError {
        ConfigError::ParseError(err.to_string())
    }
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        dotenv().ok();

        // Get the user's home directory
        let config_dir = match dirs::config_dir() {
            Some(dir) => dir.to_path_buf(),
            None => {
                return Err(ConfigError::FileNotFound(
                    "No config directory found".to_string(),
                ))
            }
        };

        // Define the config folder
        let mastermind_dir = config_dir.join("mastermind");
        if !mastermind_dir.exists() {
            match fs::create_dir_all(&mastermind_dir) {
                Ok(_) => println!("Directory created successfully at {:?}", mastermind_dir),
                Err(e) => {
                    return Err(ConfigError::FileNotFound(format!(
                        "Failed to create folder: {}",
                        e
                    )))
                }
            }
        }

        // Define config file path
        let config_file = mastermind_dir.join("config.toml");

        // If the file doesn't exist
        let document = if !config_file.exists() {
            // Create a new Document
            let mut doc = DocumentMut::new();

            // Make .toml file in table-like format
            doc["api"] = Item::Table(Default::default());
            doc["api"]["base-url"] = value("");
            doc["api"]["key"] = value("");

            doc["model"] = Item::Table(Default::default());
            doc["model"]["default"] = value("");

            // Write the document to the config file
            fs::write(&config_file, doc.to_string())?;

            doc
        } else {
            // If the file exists, load it
            let config_str = fs::read_to_string(&config_file)?;
            config_str.parse::<DocumentMut>()?
        };

        Ok(Config { document })
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), ConfigError> {
        fs::write(&path, self.document.to_string())?;
        Ok(())
    }

    pub fn get_base_url(&self) -> Option<&str> {
        self.document["api"]["base-url"]
            .as_str()
            .filter(|s| !s.is_empty())
    }

    pub fn get_api_key(&self) -> Option<&str> {
        self.document["api"]["key"]
            .as_str()
            .filter(|s| !s.is_empty())
    }

    pub fn get_default_model(&self) -> Option<&str> {
        self.document["model"]["default"]
            .as_str()
            .filter(|s| !s.is_empty())
    }
}
