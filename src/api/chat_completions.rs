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
