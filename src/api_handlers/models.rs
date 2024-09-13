use crate::json_models::language_models::ModelsResponse;

pub async fn get_model_ids_from_api(base_url: &str, key: &str) -> reqwest::Result<Vec<String>> {
    let client = reqwest::Client::new();
    let response = client
        .get(base_url.to_string() + "models")
        .bearer_auth(key)
        .send()
        .await?;

    let mut model_ids = response
        .json::<ModelsResponse>()
        .await?
        .data
        .iter()
        .map(|model| model.id.trim().to_string())
        .collect::<Vec<String>>();
    model_ids.sort();

    Ok(model_ids)
}
