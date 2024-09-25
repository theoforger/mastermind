use super::Instance;
use crate::json_models::language_models::ModelsResponse;

impl Instance {
    pub async fn get_models(&self) -> Result<ModelsResponse, Box<dyn std::error::Error>> {
        let response = self
            .client
            .get(format!("{}models", self.base_url))
            .bearer_auth(&self.key)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch model IDs from API server: {}", e))?;

        let parsed_response = response
            .json::<ModelsResponse>()
            .await
            .map_err(|e| format!("Failed to parse model IDs from API server: {}", e))?;

        Ok(parsed_response)
    }
}
