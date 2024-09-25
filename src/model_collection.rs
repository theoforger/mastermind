use crate::json_models::language_models::ModelsResponse;
use dialoguer::MultiSelect;

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
            .with_prompt("Choose language model(s)\n[Space] to select, [Enter] to confirm")
            .items(&self.model_ids)
            .interact()
            .unwrap();

        let chosen_model_ids = chosen_indexes
            .iter()
            .map(|&i| self.model_ids[i].to_string())
            .collect();

        chosen_model_ids
    }

    pub fn generate_string(&self) -> String {
        self.model_ids.join("\n")
    }

    pub fn contains(&self, model_id: &String) -> bool {
        self.model_ids.contains(model_id)
    }
}
