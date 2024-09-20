use super::Instance;
use crate::json_models::language_model::ModelsResponse;

impl Instance {
    pub async fn fetch_language_model_ids(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let response = self
            .client
            .get(format!("{}models", self.base_url))
            .bearer_auth(&self.key)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch model IDs from API server: {}", e))?;

        let mut all_model_ids = response
            .json::<ModelsResponse>()
            .await
            .map_err(|e| format!("Failed to parse model IDs from API server: {}", e))?
            .data
            .iter()
            .map(|model| model.id.trim().to_string())
            .collect::<Vec<String>>();

        all_model_ids.sort();

        Ok(all_model_ids)
    }
}
