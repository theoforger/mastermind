mod chat_completions;
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

        let base_url = Self::get_env_var("OPENAI_API_BASE_URL")?;
        let base_url = if !base_url.ends_with('/') {
            format!("{}/", base_url)
        } else {
            base_url
        };
        let key = Self::get_env_var("API_KEY")?;
        let model_id = Self::get_env_var("DEFAULT_MODEL_ID")?;

        Ok(Self {
            client: reqwest::Client::new(),
            base_url,
            key,
            model_id,
        })
    }

    fn get_env_var(var_name: &str) -> Result<String, Box<dyn std::error::Error>> {
        env::var(var_name)
            .map_err(|_| format!("Cannot read environment variable: {}", var_name).into())
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

    pub fn set_base_url(&mut self, base_url: String) {
        self.base_url = base_url;
    }
}
