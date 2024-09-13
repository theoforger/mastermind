use dotenv::dotenv;
use std::env;

pub struct ApiInstance {
    pub client: reqwest::Client,
    pub base_url: String,
    pub key: String,
}

impl ApiInstance {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();

        let base_url = match env::var("OPENAI_API_BASE_URL") {
            Ok(base_url) => base_url,
            _ => return Err("Could not read environment variable: OPENAI_API_BASE_URL".into()),
        };

        let base_url = if !base_url.ends_with('/') {
            format!("{}/", base_url)
        } else {
            base_url
        };

        let key = match env::var("API_KEY") {
            Ok(key) => key,
            _ => return Err("Could not read environment variable: API_KEY".into()),
        };

        Ok(Self {
            client: reqwest::Client::new(),
            base_url,
            key,
        })
    }
}
