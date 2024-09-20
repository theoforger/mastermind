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
pub struct Usage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
}

#[derive(Deserialize)]
pub struct ChatCompletionResponse {
    pub choices: Vec<Choice>,
    pub usage: Usage,
}
