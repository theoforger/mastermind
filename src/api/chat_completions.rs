use serde_json::json;

use super::Instance;
use crate::json::chat_completions::ChatCompletionsResponse;

const SYSTEM_PROMPT: &str = "
You are the spymaster in Codenames.
I will give you a list of [agent word], followed by a list of [avoid word].
Try to link [agent word] together.
Tro to avoid [avoid word].
Answer in this format:
[clue word], [number of agent words], [agent word], [agent word], [agent word]
...
Here are the requirements:
- Always answer in lower case.
- No special characters.
- No intro or outro.
- No explanations.
- Give 5-10 [clue word].
- Each [clue word] should link at least 2 [agent word].
";

impl Instance {
    pub async fn post_chat_completions(
        &self,
        link_words: &[String],
        avoid_words: &[String],
        model_id: &String,
    ) -> Result<ChatCompletionsResponse, Box<dyn std::error::Error>> {
        let request_body = Self::build_request_body(link_words, avoid_words, model_id);

        // Get response from API endpoint
        let response = self
            .client
            .post(format!("{}chat/completions", self.base_url))
            .bearer_auth(&self.api_key)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch clue collection from API server: {e}"))?;

        let parsed_response = response
            .json::<ChatCompletionsResponse>()
            .await
            .map_err(|e| format!("Failed to parse clues from API server: {e}"))?;

        Ok(parsed_response)
    }

    fn build_request_body(
        link_words: &[String],
        avoid_words: &[String],
        model_id: &String,
    ) -> serde_json::Value {
        // Aggregate two sets of words into one prompt
        let content = format!(
            "To Link:\n{}\n\nTo Avoid:\n{}",
            link_words.join("\n"),
            avoid_words.join("\n")
        );

        json!({
            "messages": [
                {
                    "role": "system",
                    "content": SYSTEM_PROMPT,
                },
                {
                    "role": "user",
                    "content": content
                }
            ],
            "model": model_id
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_words_from_file;
    use httpmock::prelude::*;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_post_chat_completions() {
        // Start a lightweight mock server.
        let server = MockServer::start_async().await;

        // Create a mock on the server.
        let mock = server.mock(|when, then| {
            when.method(POST).path("/chat/completions");
            then.status(200)
                .header("content-type", "application/json")
                .body_from_file("resources/tests/mock_responses/chat_completions.json");
        });

        // Create an API instance and set the base url to mock server url
        let mut api_instance = Instance::new().unwrap();
        api_instance.set_base_url(server.url("/"));

        // Get responses from mock server
        api_instance
            .post_chat_completions(&Vec::<String>::new(), &Vec::<String>::new(), &String::new())
            .await
            .unwrap();
        mock.assert();
    }

    #[test]
    fn test_build_request_body() {
        // Mock input data
        let link_words = vec!["link1".to_string(), "link2".to_string()];
        let avoid_words = vec!["avoid1".to_string(), "avoid2".to_string()];
        let model_id = "model".to_string();

        // Assign result to the result of build_request_body() method
        let result = Instance::build_request_body(&link_words, &avoid_words, &model_id);

        // Format expected content
        let expected_content = format!(
            "To Link:\n{}\n\nTo Avoid:\n{}",
            link_words.join("\n"),
            avoid_words.join("\n")
        );

        // Mock expected output
        let expected = json!(
            {
            "messages": [
                {
                    "role": "system",
                    "content": SYSTEM_PROMPT,
                },
                {
                    "role": "user",
                    "content": expected_content
                }
            ],
            "model": model_id
        });
        assert_eq!(expected, result);
    }
}
