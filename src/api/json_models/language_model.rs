use serde::Deserialize;

#[derive(Deserialize)]
pub struct LanguageModel {
    pub id: String,
}

#[derive(Deserialize)]
pub struct ModelsResponse {
    pub data: Vec<LanguageModel>,
}
