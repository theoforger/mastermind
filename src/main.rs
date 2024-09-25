use std::env;
use clap::Parser;
use dotenv::dotenv;
use mastermind::api::Instance;
use mastermind::clue::ClueCollection;
use mastermind::json_models::chat_completions::ChatCompletionsResponse;
use mastermind::model_collection::ModelCollection;
use mastermind::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read arguments and environment variables
    let args = Args::parse();
    dotenv().ok();

    // Create an API instance and get all available models
    let mut api_instance = Instance::new()?;
    let models_response = api_instance.get_models().await?;
    let model_collection = ModelCollection::new(models_response);

    // If -g is set, call the models API endpoint instead
    if args.get {
        println!("{}", model_collection.generate_string());
        return Ok(());
    }

    // If -m is set, use a preferred language model
    if let Some(model_ids) = args.model {
        if model_ids[0] == "interactive" {
            let selected_model = model_collection.prompt_selection()[0].to_string();
            api_instance.set_model_id(selected_model).await?;
        } else {
            let selected_model = env::var("DEFAULT_MODEL_ID")
                .map_err(|_| "Cannot read environment variable: DEFAULT_MODEL_ID".into())?;
            api_instance.set_model_id(selected_model).await?;
        }
    }

    // Attempt to read words from the two files
    let link_words = read_words_from_file(args.to_link.unwrap())?;
    let avoid_words = read_words_from_file(args.to_avoid.unwrap())?;

    // Get responses
    // If -m is set, use a preferred language model(s)
    // Otherwise, call the API straight away
    let responses = match args.model {
        Some(model_ids) => {
            let mut responses: Vec<ChatCompletionsResponse> = vec![];
            for model_id in model_ids {
                api_instance.set_model_id(model_id).await?;
                let response = api_instance
                    .post_chat_completions(&link_words, &avoid_words)
                    .await?;
                responses.push(response);
            }
            responses
        }
        None => vec![
            api_instance
                .post_chat_completions(&link_words, &avoid_words)
                .await?,
        ],
    };

    // Build ClueCollection from the responses
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
