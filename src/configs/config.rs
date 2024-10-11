use dotenv::dotenv;
use std::fs;
use std::path::Path;
use toml_edit::{value, DocumentMut, Item, Table};

use crate::configs::config_error::ConfigError;

pub struct Config {
    document: DocumentMut,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        dotenv().ok();

        // Get the user's home directory
        let Some(config_dir) = dirs::config_dir() else {
            return Err(ConfigError::FileNotFound(
                "No config directory found".to_string(),
            ));
        };

        // Define the config folder
        let mastermind_dir = config_dir.join("mastermind");
        if !mastermind_dir.exists() {
            match fs::create_dir_all(&mastermind_dir) {
                Ok(()) => println!("Directory created successfully at {mastermind_dir:?}"),
                Err(e) => {
                    return Err(ConfigError::FileNotFound(format!(
                        "Failed to create folder: {e}"
                    )))
                }
            }
        }

        // Define config file path
        let config_file = mastermind_dir.join("config.toml");

        // Read or create a document
        let document = match fs::read_to_string(&config_file) {
            Ok(content) if !content.is_empty() => content.parse::<DocumentMut>()?,
            _ => {
                let mut doc = DocumentMut::new();

                // Make .toml file in table-like format
                doc["api"] = Item::Table(Table::new());
                doc["api"]["base-url"] = value("");
                doc["api"]["key"] = value("");

                doc["model"] = Item::Table(Table::new());
                doc["model"]["default"] = value("");

                // Write the document to the config file
                fs::write(&config_file, doc.to_string())?;

                doc
            }
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
