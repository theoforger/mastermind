use std::env;
use std::path::PathBuf;

use clap::Parser;
use dotenv::dotenv;

use mastermind::api_handlers::chat_completions::*;
use mastermind::api_handlers::models::*;
use mastermind::*;

/// Mastermind - An LLM-powered CLI tool to help you be a better spymaster in Codenames
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Get available language json_models from API
    #[arg(short, long = "get-models")]
    get: bool,

    /// Specify a language model
    #[arg(short, long = "set-model")]
    model: Option<String>,

    /// Path to a file containing words to link together - the words from your team
    #[arg(required_unless_present = "get")]
    to_link: Option<PathBuf>,

    /// Path to a file containing words to avoid - opponent's words, neutral words, and the assassin word
    #[arg(required_unless_present = "get")]
    to_avoid: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read arguments and environment variables
    let args = Args::parse();
    dotenv().ok();

    // Get all model IDs for future reference
    let model_ids = get_model_ids_from_api().await?;

    // If -g is set, call the models API endpoint instead
    if args.get {
        println!("{}", model_ids.join("\n"));
        return Ok(());
    }

    // If -m is set, use a preferred language model
    // Otherwise, use the default
    let model_id = if args.model.is_some() {
        args.model.unwrap()
    } else {
        match env::var("DEFAULT_MODEL_ID") {
            Ok(id) => id,
            _ => return Err("Could not read environment variable: OPENAI_API_BASE_URL. Use -m to specify a language model".into())
        }
    };

    // Abort the program if the chosen model is not valid
    if !model_ids.contains(&model_id) {
        return Err(format!(
            "{} is not a valid language model from your provider",
            model_id
        )
        .into());
    }

    // Get clues from API
    let link_words = read_words_from_file(args.to_link.unwrap());
    let avoid_words = read_words_from_file(args.to_avoid.unwrap());
    let clues = get_clues_from_api(link_words, avoid_words, &model_id).await?;

    if clues.is_empty() {
        println!("The language model didn't return any useful clues. Maybe try again?");
    } else {
        let output = clues.join("\n");
        println!("{}", output);
    }

    Ok(())
}
