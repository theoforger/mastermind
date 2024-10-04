use dotenv::dotenv;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use toml_edit::{value, DocumentMut, Item};

#[derive(Debug)]
pub enum ConfigError {
    FileNotFound(String),
    ParseError(String),
    IoError(String),
}

pub struct Config {
    pub document: DocumentMut,
    pub path: PathBuf,
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
        let config_dir = dirs::config_dir().unwrap().to_path_buf();

        // Define the config folder
        let master_mind_folder = config_dir.join("mastermind");
        match fs::create_dir_all(&master_mind_folder) {
            Ok(_) => println!("Folder created successfully at {:?}", master_mind_folder),
            Err(e) => eprintln!("Failed to create folder: {}", e),
        }

        // Define config file path
        let config_file = master_mind_folder.join(".config.toml");

        // Debug, left, in case you are having hard time finding a file
        // println!("config_file path is : {:?}", config_file);

        // If the file doesn't exist
        let document = if !config_file.exists() {
            // Create a new Document
            let mut doc = DocumentMut::new();

            // Make .toml file in table-like format
            doc["api"] = Item::Table(Default::default());

            doc["api"]["base"] = value("");
            doc["api"]["key"] = value("");

            // Write the document to the config file
            fs::write(&config_file, doc.to_string())?;

            doc
        } else {
            // If the file exists, load it
            let config_str = fs::read_to_string(&config_file)?;
            config_str.parse::<DocumentMut>()?
        };

        Ok(Config {
            document,
            path: config_file,
        })
    }
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let config_str = fs::read_to_string(&path)?;
        let document = config_str.parse::<DocumentMut>()?;
        Ok(Config {
            document,
            path: path.as_ref().to_path_buf(),
        })
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
