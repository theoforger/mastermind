use clap::Parser;

use mastermind::api::Instance;
use mastermind::clue::ClueCollection;
use mastermind::json_models::chat_completions::ChatCompletionsResponse;
use mastermind::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read arguments and environment variables
    let args = Args::parse();

    // Create an API instance
    let mut api_instance = Instance::new()?;

    // If -g is set, call the models API endpoint instead
    if args.get {
        println!("{}", api_instance.get_models().await?.join("\n"));
        return Ok(());
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
