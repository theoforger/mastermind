mod chat_completions;
mod language_models;

use dotenv::dotenv;
use std::env;
use crate::model_collection::ModelCollection;

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

    pub async fn set_model_id(
        &mut self,
        model_id: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Return Error if the chosen model is not valid
        let models_response = self.get_models().await?;
        let model_collection = ModelCollection::new(models_response);
        
        if !model_collection.contains(&model_id) {
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
