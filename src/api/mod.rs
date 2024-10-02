mod chat_completions;
mod models;

// use std::env;
use crate::config;

use config::ConfigError;

pub struct Instance {
    client: reqwest::Client,
    base_url: String,
    key: String,
}

impl Instance {
    pub fn new() -> Result<Self, ConfigError> {

        let config = config::Config::from_file("config.toml")?;

        let base_url = match config.get_base_url() {
            Some(url) => url.to_string(),
            None=> {
                eprintln!("Base URL not found in configuration.");
                return Err(ConfigError::ParseError("Base URL not found".to_string()));
            }
        };

        let base_url = if !base_url.ends_with('/') {
            format!("{}/", base_url)
        } else {
            base_url
        };
        let key = match config.get_api_key(){
            Some(key) => key.to_string(),
            None => {
                eprintln!("API key not found in configuration.");
                return Err(ConfigError::ParseError("API key not found".to_string()));
            }
        };

        Ok(Self {
            client: reqwest::Client::new(),
            base_url,
            key,
        })
    }

    // Commented as never used
    // fn get_env_var(var_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    //     env::var(var_name)
    //         .map_err(|_| format!("Cannot read environment variable: {}", var_name).into())
    // }

    pub fn set_base_url(&mut self, base_url: String) {
        self.base_url = base_url;
    }
}
