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

        let base_url = env::var("OPENAI_API_BASE_URL")
            .map_err(|_| "Cannot read environment variable: OPENAI_API_BASE_URL")?;

        let base_url = if !base_url.ends_with('/') {
            format!("{}/", base_url)
        } else {
            base_url
        };

        let key = env::var("API_KEY")
            .map_err(|_| "Cannot read environment variable: API_KEY")?;

        Ok(Self {
            client: reqwest::Client::new(),
            base_url,
            key,
        })
    }
}
