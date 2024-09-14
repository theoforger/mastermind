use super::api_instance::ApiInstance;
use super::json_models::chat_completion::ChatCompletionResponse;
use crate::clue::{Clue, ClueCollection};
use serde_json::json;

const SYSTEM_PROMPT: &str = r#"You are the spymaster in Codenames.
I will give you a list of words to link together, followed by a list of words to avoid.
Respond with a list of clue words followed by the words they are supposed to link together.
With each clue word, try to link as many words as possible.
Here are the requirements:
- Always answer in lower case.
- Give 5 to 10 clue word options.
- Do not give repeated clue words.
- Never give any intro, outro or explanation.
- Only give the words themselves. Do not add anything else.
- Answer in this format:
    [clue] [number of agent words] - [agent word] [agent word] [agent word]
    [clue] [number of agent words] - [agent word] [agent word] [agent word]
    ...
"#;

fn build_request_body_for_clues(
    link_words: Vec<String>,
    avoid_words: Vec<String>,
    model_id: &str,
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

pub async fn get_clues_from_api(
    link_words: Vec<String>,
    avoid_words: Vec<String>,
    model_id: &str,
) -> Result<ClueCollection, Box<dyn std::error::Error>> {
    let api_instance = ApiInstance::new()?;
    let request_body = build_request_body_for_clues(link_words, avoid_words, model_id);

    // Get response from API endpoint
    let response = api_instance
        .client
        .post(api_instance.base_url.to_string() + "chat/completions")
        .bearer_auth(api_instance.key)
        .json(&request_body)
        .send()
        .await?;

    // Deserialize the response
    let clue_strings = response.json::<ChatCompletionResponse>().await?.choices[0]
        .message
        .content
        .lines()
        .map(|line| line.trim().to_string())
        .collect::<Vec<String>>();

    // Build clues
    let mut clue_collection = ClueCollection::new(clue_strings);
    clue_collection.sort();

    Ok(clue_collection)
}
