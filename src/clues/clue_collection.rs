use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, CellAlignment, ContentArrangement, Table};

use crate::clues::clue::Clue;
use crate::json::chat_completions::{ChatCompletionsResponse, Usage};

pub struct ClueCollection {
    clues: Vec<Clue>,
    usage: Usage,
}

impl ClueCollection {
    /// Create a new instance of `ClueCollection` from `Vec<ChatCompletionsResponse>`
    pub fn new(responses: Vec<ChatCompletionsResponse>) -> Self {
        let mut clues: Vec<Clue> = vec![];

        let mut usage = Usage {
            prompt_tokens: 0,
            completion_tokens: 0,
            total_tokens: 0,
        };

        // Aggregate clues and token usage information
        for response in responses {
            for choice in response.choices {
                for line in choice.message.content.lines() {
                    if let Some(clue) = Clue::new(line.trim(), response.model.clone()) {
                        clues.push(clue);
                    }
                }
            }
            usage.prompt_tokens += response.usage.prompt_tokens;
            usage.completion_tokens += response.usage.completion_tokens;
            usage.total_tokens += response.usage.total_tokens;
        }

        // Sort the clues by the number of words they link together
        clues.sort_by(|a, b| b.count.cmp(&a.count));

        // Return
        Self { clues, usage }
    }

    pub fn is_empty(&self) -> bool {
        self.clues.is_empty()
    }

    fn generate_table(&self) -> Table {
        let mut table = Table::new();

        // Set up header and styles
        table
            .set_header(vec![
                Cell::new("Clue")
                    .add_attribute(Attribute::Bold)
                    .set_alignment(CellAlignment::Center),
                Cell::new("Count")
                    .add_attribute(Attribute::Bold)
                    .set_alignment(CellAlignment::Center),
                Cell::new("Linked Words")
                    .add_attribute(Attribute::Bold)
                    .set_alignment(CellAlignment::Center),
                Cell::new("Source")
                    .add_attribute(Attribute::Bold)
                    .set_alignment(CellAlignment::Center),
            ])
            .set_content_arrangement(ContentArrangement::Dynamic)
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS);

        // Add rows
        for clue in &self.clues {
            table.add_row(vec![
                clue.clue_word.clone(),
                clue.count.to_string(),
                clue.linked_words.join(", "),
                clue.source.clone(),
            ]);
        }

        // Center the second column
        let second_column = table
            .column_mut(1)
            .expect("The table should have more than 2 columns");
        second_column.set_cell_alignment(CellAlignment::Center);

        table
    }

    pub fn display_token_info(&self) {
        eprintln!(
            "\nToken Usage:\n\
            ----------------------\n\
            Prompt Tokens: {}\n\
            Completion Tokens: {}\n\
            ----------------------\n\
            Total Tokens: {}",
            self.usage.prompt_tokens, self.usage.completion_tokens, self.usage.total_tokens
        );
    }
}

impl std::fmt::Display for ClueCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.generate_table())
    }
}

#[cfg(test)]
impl ClueCollection {
    pub fn generate_list(&self) -> String {
        let mut list = String::new();
        for clue in &self.clues {
            let clue_string = format!(
                "{} {} - {}\n",
                clue.clue_word,
                clue.count,
                clue.linked_words.join(", ")
            );
            list.push_str(clue_string.as_str());
        }
        list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::api::Instance;
    use httpmock::prelude::*;
    use std::fs;
    #[tokio::test]
    async fn test_new() {
        // Start a lightweight mock server.
        let server = MockServer::start_async().await;

        // Create a mock on the server.
        let mock = server.mock(|when, then| {
            when.method(POST).path("/chat/completions");
            then.status(200)
                .header("content-type", "application/json")
                .body_from_file("resources/tests/mock_responses/chat_completions.json");
        });

        // Create an API instance and set the base url to mock server url
        let mut api_instance = Instance::new().unwrap_or_default();
        api_instance.set_base_url(server.url("/"));

        // Get responses from mock server
        let responses = vec![api_instance
            .post_chat_completions(&Vec::<String>::new(), &Vec::<String>::new(), &String::new())
            .await
            .unwrap()];
        mock.assert();

        // Compare outputs
        let output = ClueCollection::new(responses).generate_list();
        let expected_output =
            fs::read_to_string("resources/tests/expected_outputs/chat_completions.txt").unwrap();
        assert_eq!(output, expected_output);
    }
}
