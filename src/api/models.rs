use super::Instance;
use crate::json::models::ModelsResponse;

impl Instance {
    pub async fn get_models(&self) -> Result<ModelsResponse, Box<dyn std::error::Error>> {
        let response = self
            .client
            .get(format!("{}models", self.base_url))
            .bearer_auth(&self.api_key)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch model IDs from API server: {e}"))?;

        let parsed_response = response
            .json::<ModelsResponse>()
            .await
            .map_err(|e| format!("Failed to parse model IDs from API server: {e}"))?;

        Ok(parsed_response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;

    #[tokio::test]
    async fn test_get_models() {
        // Start a lightweight mock server.
        let server = MockServer::start_async().await;

        // Create a mock on the server.
        let mock = server.mock(|when, then| {
            when.method(GET).path("/models");
            then.status(200)
                .header("content-type", "application/json")
                .body_from_file("resources/tests/mock_responses/models.json");
        });

        // Create an API instance and set the base url to mock server url
        let mut api_instance = Instance::new().unwrap_or_default();
        api_instance.set_base_url(server.url("/"));

        // Get response from mock server
        api_instance.get_models().await.unwrap();
        mock.assert();
    }
}
