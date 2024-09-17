use super::json_models::language_model::ModelsResponse;
use super::Instance;

impl Instance {
    pub async fn fetch_all_model_ids(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let response = self
            .client
            .get(format!("{}models", self.base_url))
            .bearer_auth(&self.key)
            .send()
            .await
            .map_err(|_| "Failed to fetch model IDs from API server")?;

        let mut all_model_ids = response
            .json::<ModelsResponse>()
            .await
            .map_err(|_| "Failed to parse API response from server when fetching model IDs")?
            .data
            .iter()
            .map(|model| model.id.trim().to_string())
            .collect::<Vec<String>>();

        all_model_ids.sort();

        Ok(all_model_ids)
    }
}
