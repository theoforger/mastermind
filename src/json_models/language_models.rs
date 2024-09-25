use serde::Deserialize;

#[derive(Deserialize)]
pub struct Model {
    pub id: String,
}

#[derive(Deserialize)]
pub struct ModelsResponse {
    pub data: Vec<Model>,
}