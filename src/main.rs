use clap::Parser;
use dotenv::dotenv;
use std::env;

use mastermind::*;

use clue::ClueCollection;
use model::ModelCollection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read arguments and environment variables
    let args = Args::parse();
    dotenv().ok();

    // Create an API instance and get all available models
    let api_instance = api::Instance::new()?;
    let models_response = api_instance.get_models().await?;
    let model_collection = ModelCollection::new(models_response);

    // If -g is set, call the models API endpoint instead
    if args.get {
        model_collection.display_list();
        return Ok(());
    }

    // Read words from the two files
    let link_words = read_words_from_file(args.to_link.unwrap())?;
    let avoid_words = read_words_from_file(args.to_avoid.unwrap())?;

    // If -m is present and has values, use the preferred language models
    // If -m is present but doesn't have a value, prompt selection menu
    // If -m is not present, use the default from environment variable
    let selected_model_ids = match args.models {
        Some(model_ids) => {
            if model_ids[0] == "interactive" {
                model_collection.prompt_selection()
            } else {
                model_ids
            }
        }
        None => vec![env::var("DEFAULT_MODEL_ID")
            .map_err(|_| "Cannot read environment variable: DEFAULT_MODEL_ID")?],
    };

    // Aggregate responses from each language model and build ClueCollection
    let mut responses = vec![];
    for model_id in &selected_model_ids {
        model_collection.validate_model_id(model_id)?;
        let response = api_instance
            .post_chat_completions(&link_words, &avoid_words, model_id)
            .await?;
        responses.push(response);
    }
    let clue_collection = ClueCollection::new(responses);

    // Output
    if clue_collection.is_empty() {
        println!("The language model didn't return any useful clues. Maybe try again?");
    } else if let Some(output_path) = args.output {
        println!("Writing to file '{}'...", output_path.display());
        write_content_to_file(output_path, clue_collection.generate_table())?;
    } else {
        clue_collection.display_table();
    }

    // If -t is set, output token usage information
    if args.token {
        clue_collection.display_token_info();
    }

    Ok(())
}
