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
                Ok(()) => println!("Config directory created at {mastermind_dir:?}"),
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
                println!("Config file not found or empty. Creating one...");
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_new() {
        // Create a temporary directory
        let temp_dir = tempdir().unwrap();
        let config_dir = temp_dir.path().join("mastermind");
        assert!(!config_dir.exists());

        // Override config home
        std::env::set_var("XDG_CONFIG_HOME", temp_dir.path().to_str().unwrap());

        // Create a config
        let config_result = Config::new();
        assert!(config_result.is_ok());
        assert!(config_dir.exists());

        // Check if config.toml exists
        let config_file = config_dir.join("config.toml");
        assert!(config_file.exists());

        // Check the content
        let content = fs::read_to_string(config_file).unwrap();
        assert!(content.contains("[api]"));
        assert!(content.contains("base-url"));
        assert!(content.contains("key"));
        assert!(content.contains("[model]"));
        assert!(content.contains("default"));
    }
}
