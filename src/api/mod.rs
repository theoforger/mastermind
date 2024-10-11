mod chat_completions;
mod models;

use crate::config::Config;
use dotenv::dotenv;
use std::env;

pub struct Instance {
    client: reqwest::Client,
    base_url: String,
    api_key: String,
}

impl Instance {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config = Config::new()?;
        dotenv().ok();

        let base_url = match env::var("OPENAI_API_BASE_URL") {
            Ok(url) => url,
            Err(_) => {
                if let Some(config_url) = config.get_base_url() {
                    config_url.to_string()
                } else {
                    return Err(
                        "Could not find base URL in config file or environment variable".into(),
                    );
                }
            }
        };

        let base_url = if !base_url.ends_with('/') {
            format!("{}/", base_url)
        } else {
            base_url
        };

        let api_key = match env::var("API_KEY") {
            Ok(key) => key,
            Err(_) => {
                if let Some(config_key) = config.get_api_key() {
                    config_key.to_string()
                } else {
                    return Err(
                        "Could not find API Key in config file or environment variable".into(),
                    );
                }
            }
        };

        Ok(Self {
            client: reqwest::Client::new(),
            base_url,
            api_key,
        })
    }
}

#[cfg(test)]
impl Instance {
    pub(crate) fn set_base_url(&mut self, base_url: String) {
        self.base_url = base_url;
    }
}
