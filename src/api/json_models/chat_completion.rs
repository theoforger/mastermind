use serde::Deserialize;

#[derive(Deserialize)]
pub struct Message {
    pub content: String,
}

#[derive(Deserialize)]
pub struct Choice {
    pub message: Message,
}

#[derive(Deserialize)]
pub struct ChatCompletionResponse {
    pub choices: Vec<Choice>,
}
