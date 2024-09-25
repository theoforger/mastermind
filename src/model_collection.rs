use crate::json_models::language_models::ModelsResponse;
use dialoguer::MultiSelect;

struct ModelCollection {
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
            .with_prompt("What do you choose?")
            .items(&self.model_ids)
            .interact()
            .unwrap();

        let chosen_model_ids = chosen_indexes
            .iter()
            .map(|&i| self.model_ids[i].to_string())
            .collect();

        chosen_model_ids
    }
}
