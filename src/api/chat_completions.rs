use super::json_models::chat_completion::{ChatCompletionResponse};
use super::Instance;
use crate::clue::ClueCollection;
use serde_json::json;



const SYSTEM_PROMPT: &str = r#"
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
"#;

impl Instance {
    pub async fn fetch_clue_collection(
        &self,
        link_words: Vec<String>,
        avoid_words: Vec<String>,
    ) -> Result<ClueCollection, Box<dyn std::error::Error>> {
        let request_body = self.build_request_body(link_words, avoid_words);

        // Get response from API endpoint
        let response = self
            .client
            .post(format!("{}chat/completions", self.base_url))
            .bearer_auth(&self.key)
            .json(&request_body)
            .send()
            .await
            .map_err(|_| "Failed to fetch clue collection from API server")?;

        let parsed_response = response
            .json::<ChatCompletionResponse>()
            .await
            .map_err(|_| "Failed to parse clues from API server")?;

        // Extract usage information from the parsed response
        let token_usage = parsed_response.usage;


        // Extract clue strings from the parsed response
        let clue_strings = parsed_response
            .choices
            .get(0)
            .ok_or("No choices returned from API")?
            .message
            .content
            .lines()
            .map(|line| line.trim().to_string())
            .collect::<Vec<String>>();

        // Build clues
        let clue_collection = ClueCollection::new(clue_strings, token_usage);

        Ok(clue_collection)
    }

    fn build_request_body(
        &self,
        link_words: Vec<String>,
        avoid_words: Vec<String>,
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
            "model": self.model_id
        })
    }
}
