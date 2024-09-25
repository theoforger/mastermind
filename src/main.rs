use clap::Parser;
use dotenv::dotenv;
use std::env;
use std::error::Error;

use mastermind::*;

use clue::ClueCollection;
use model::ModelCollection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Read arguments and environment variables
    let args = Args::parse();
    dotenv().ok();

    // Create an API instance and get all available models from API
    let api_instance = api::Instance::new()?;
    let models_response = api_instance.get_models().await?;
    let model_collection = ModelCollection::new(models_response);

    // If -g is set, display models and exit the program
    if args.get {
        model_collection.display_list();
        return Ok(());
    }

    // Determine selected models
    let selected_model_ids = select_models(&args, &model_collection)?;

    // Various API calls and then build ClueCollection
    let clue_collection =
        obtain_clue_collection(&args, api_instance, model_collection, &selected_model_ids).await?;

    // Output
    handle_output(&args, clue_collection)?;

    Ok(())
}

/// If -m is present and has values, use the preferred language models.
/// If -m is present but doesn't have a value, prompt selection menu.
/// If -m is not present, use the default from environment variable.
fn select_models(
    args: &Args,
    model_collection: &ModelCollection,
) -> Result<Vec<String>, Box<dyn Error>> {
    let selected_model_ids = match &args.models {
        Some(model_ids) => {
            if model_ids[0] == "interactive" {
                model_collection.prompt_selection()
            } else {
                model_ids.to_owned()
            }
        }
        None => vec![env::var("DEFAULT_MODEL_ID")
            .map_err(|_| "Cannot read environment variable: DEFAULT_MODEL_ID")?],
    };
    Ok(selected_model_ids)
}

async fn obtain_clue_collection(
    args: &Args,
    api_instance: api::Instance,
    model_collection: ModelCollection,
    selected_model_ids: &Vec<String>,
) -> Result<ClueCollection, Box<dyn Error>> {
    // Read words from the two files
    let link_words = read_words_from_file(args.to_link.as_ref().unwrap())?;
    let avoid_words = read_words_from_file(args.to_avoid.as_ref().unwrap())?;

    // Aggregate responses from each language model
    let mut responses = vec![];
    for model_id in selected_model_ids {
        // Validate each selected model
        model_collection.validate_model_id(model_id)?;
        // Get response from API
        let response = api_instance
            .post_chat_completions(&link_words, &avoid_words, model_id)
            .await?;
        responses.push(response);
    }

    // Build ClueCollection
    let clue_collection = ClueCollection::new(responses);
    Ok(clue_collection)
}

fn handle_output(args: &Args, clue_collection: ClueCollection) -> Result<(), Box<dyn Error>> {
    if clue_collection.is_empty() {
        println!("The language model didn't return any useful clues. Maybe try again?");
    } else if let Some(output_path) = &args.output {
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
