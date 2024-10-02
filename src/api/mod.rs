mod chat_completions;
mod models;

use dotenv::dotenv;
use std::env;

pub struct Instance {
    client: reqwest::Client,
    base_url: String,
    key: String,
}

impl Instance {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();

        let base_url = Self::get_env_var("OPENAI_API_BASE_URL")?;
        let base_url = if !base_url.ends_with('/') {
            format!("{}/", base_url)
        } else {
            base_url
        };
        let key = Self::get_env_var("API_KEY")?;

        Ok(Self {
            client: reqwest::Client::new(),
            base_url,
            key,
        })
    }

    fn get_env_var(var_name: &str) -> Result<String, Box<dyn std::error::Error>> {
        env::var(var_name)
            .map_err(|_| format!("Cannot read environment variable: {}", var_name).into())
    }
}

#[cfg(test)]
impl Instance {
    pub(crate) fn set_base_url(&mut self, base_url: String) {
        self.base_url = base_url;
    }
}
