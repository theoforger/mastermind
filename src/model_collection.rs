use dialoguer::MultiSelect;
use std::fmt::{Display, Formatter};

use crate::json::models::ModelsResponse;

pub struct ModelCollection {
    model_ids: Vec<String>,
}

impl ModelCollection {
    pub fn new(response: &ModelsResponse) -> Self {
        let mut model_ids: Vec<String> = vec![];
        response
            .data
            .iter()
            .for_each(|model| model_ids.push(model.id.trim().to_string()));

        model_ids.sort();

        Self { model_ids }
    }

    pub fn prompt_selection(&self) -> Vec<String> {
        let chosen_indexes = MultiSelect::new()
            .with_prompt("[Space] to select, [Enter] to confirm\nYour choice(s)")
            .items(&self.model_ids)
            .interact()
            .unwrap();

        let chosen_model_ids = chosen_indexes
            .iter()
            .map(|&i| self.model_ids[i].to_string())
            .collect();

        chosen_model_ids
    }

    pub fn validate_model_id(&self, model_id: &String) -> Result<(), Box<dyn std::error::Error>> {
        if !self.model_ids.contains(model_id) {
            return Err(
                format!("{model_id} is not a valid language model from your provider").into(),
            );
        }

        Ok(())
    }
}

impl Display for ModelCollection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.model_ids.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::api::Instance;
    use httpmock::prelude::*;
    use std::fs;

    #[tokio::test]
    async fn test_new() {
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
        let mut api_instance = Instance::new().unwrap();
        api_instance.set_base_url(server.url("/"));

        // Get response from mock server
        let response = ModelCollection::new(&api_instance.get_models().await.unwrap());
        mock.assert();

        // Compare outputs
        let output = response.to_string();
        let expected_output =
            fs::read_to_string("resources/tests/expected_outputs/models.txt").unwrap();
        assert_eq!(output, expected_output);
    }
}
