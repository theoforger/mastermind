use crate::json::models::ModelsResponse;
use dialoguer::MultiSelect;
use std::fmt::{Display, Formatter};

pub struct ModelCollection {
    model_ids: Vec<String>,
}

impl ModelCollection {
    pub fn new(response: ModelsResponse) -> Self {
        let mut model_ids: Vec<String> = vec![];
        response
            .data
            .iter()
            .for_each(|model| model_ids.push(model.id.trim().to_string()));

        model_ids.sort();

        Self { model_ids }
    }

    pub fn prompt_selection(&self) -> Vec<String> {
        let chosen_indexes = MultiSelect::new()
            .with_prompt("[Space] to select, [Enter] to confirm\nYour choice(s)")
            .items(&self.model_ids)
            .interact()
            .unwrap();

        let chosen_model_ids = chosen_indexes
            .iter()
            .map(|&i| self.model_ids[i].to_string())
            .collect();

        chosen_model_ids
    }

    pub fn validate_model_id(&self, model_id: &String) -> Result<(), Box<dyn std::error::Error>> {
        if !self.model_ids.contains(model_id) {
            return Err(format!(
                "{} is not a valid language model from your provider",
                model_id
            )
            .into());
        }

        Ok(())
    }
}

impl Display for ModelCollection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.model_ids.join("\n"))
    }
}
