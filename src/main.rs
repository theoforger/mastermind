use std::env;

use clap::Parser;
use dotenv::dotenv;

use mastermind::api::Instance;
use mastermind::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read arguments and environment variables
    let args = Args::parse();
    dotenv().ok();

    // Create an API instance
    let mut api_instance = Instance::new()?;

    // If -g is set, call the models API endpoint instead
    if args.get {
        println!("{}", api_instance.fetch_all_model_ids().await?.join("\n"));
        return Ok(());
    }

    // If -m is set, use a preferred language model
    // Otherwise, set the default
    let model_id = match args.model {
        Some(id) => id,
        None => env::var("DEFAULT_MODEL_ID").map_err(|_| {
            "Could not read environment variable: DEFAULT_MODEL_ID. Use -m to specify a language model"
        })?,
    };
    api_instance.set_model_id(model_id).await?;

    // Attempt to read words from the two files
    let link_words = read_words_from_file(args.to_link.unwrap()).map_err(|e| e.to_string())?;
    let avoid_words = read_words_from_file(args.to_avoid.unwrap()).map_err(|e| e.to_string())?;

    // Get clues from API
    let clue_collection = api_instance
        .fetch_clue_collection(link_words, avoid_words)
        .await?;

    // Output
    if clue_collection.is_empty() {
        println!("The language model didn't return any useful clues. Maybe try again?");
    } else {
        clue_collection.display();
    }

    Ok(())
}
