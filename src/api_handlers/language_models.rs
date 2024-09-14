use super::api_instance::ApiInstance;
use super::json_models::language_model::ModelsResponse;

pub async fn get_model_ids_from_api() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let api_instance = ApiInstance::new()?;

    let response = api_instance
        .client
        .get(api_instance.base_url + "models")
        .bearer_auth(api_instance.key)
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
