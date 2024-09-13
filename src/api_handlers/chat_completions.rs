use crate::json_models::chat_completion::ChatCompletionResponse;
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

fn generate_prompt(link_words: Vec<String>, avoid_words: Vec<String>) -> String {
    format!(
        "To Link:\n{}\n\nTo Avoid:\n{}",
        link_words.join("\n"),
        avoid_words.join("\n")
    )
}

/// Remove invalid clues and sort the clues by the number of words they link together
fn clean_up_clues(clues: &mut Vec<String>) {
    // Remove LLM hallucination and clues that only link one word
    clues.retain(|clue| {
        let words: Vec<&str> = clue.split_whitespace().collect();
        if words.len() > 4 {
            if let Ok(count) = words[1].parse::<usize>() {
                if words.len() == count + 3 {
                    return true;
                }
            }
        }
        false
    });

    // Sort the clues by the number of words they link together
    clues.sort_by(|a, b| {
        let a_words: Vec<&str> = a.split_whitespace().collect();
        let b_words: Vec<&str> = b.split_whitespace().collect();
        b_words[1].cmp(&a_words[1])
    });
}

pub fn build_request_body_for_clues(
    link_words: Vec<String>,
    avoid_words: Vec<String>,
    model_id: &str,
) -> serde_json::Value {
    json!({
        "messages": [
            {
                "role": "system",
                "content": SYSTEM_PROMPT,
            },
            {
                "role": "user",
                "content": generate_prompt(link_words, avoid_words),
            }
        ],
        "model": model_id
    })
}

pub async fn get_clues_from_api(
    base_url: &str,
    key: &str,
    body: serde_json::Value,
) -> reqwest::Result<Vec<String>> {
    // Create a new client
    let client = reqwest::Client::new();

    // Get response from API endpoint
    let response = client
        .post(base_url.to_string() + "chat/completions")
        .bearer_auth(key)
        .json(&body)
        .send()
        .await?;

    // Deserialize the response
    let mut clues = response.json::<ChatCompletionResponse>().await?.choices[0]
        .message
        .content
        .lines()
        .map(|line| line.trim().to_string())
        .collect::<Vec<String>>();

    // Clean up
    clean_up_clues(&mut clues);

    Ok(clues)
}
