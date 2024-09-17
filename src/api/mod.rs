mod chat_completions;
mod json_models;
mod language_models;

use dotenv::dotenv;
use std::env;

pub struct Instance {
    client: reqwest::Client,
    base_url: String,
    key: String,
    model_id: String,
}

impl Instance {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();

        let base_url = env::var("OPENAI_API_BASE_URL")
            .map_err(|_| "Cannot read environment variable: OPENAI_API_BASE_URL")?;

        let base_url = if !base_url.ends_with('/') {
            format!("{}/", base_url)
        } else {
            base_url
        };

        let key = env::var("API_KEY").map_err(|_| "Cannot read environment variable: API_KEY")?;

        Ok(Self {
            client: reqwest::Client::new(),
            base_url,
            key,
            model_id: "".to_string(),
        })
    }

    pub async fn set_model_id(
        &mut self,
        model_id: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Return Error if the chosen model is not valid
        let valid_model_ids = self.fetch_all_model_ids().await?;
        if !valid_model_ids.contains(&model_id) {
            return Err(format!(
                "{} is not a valid language model from your provider",
                model_id
            )
            .into());
        }

        self.model_id = model_id;
        Ok(())
    }
}
