use clap::Parser;
use dotenv::dotenv;
use std::env;
use std::path::PathBuf;

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

    // API Setup
    let api_key = env::var("API_KEY")?;
    let mut base_url = env::var("OPENAI_API_BASE_URL")?;
    if !base_url.ends_with('/') {
        base_url.push('/');
    }

    // If -g is set, call the models API endpoint instead
    if args.get {
        let output = get_model_ids_from_api(&base_url, &api_key)
            .await?
            .join("\n");
        println!("{}", output);
        return Ok(());
    }

    // If -m is set, use a preferred language model
    // Otherwise, use the default
    let model_id = if args.model.is_some() {
        args.model.unwrap()
    } else {
        env::var("DEFAULT_MODEL_ID")?
    };

    // Ready the words
    let link_words = read_words_from_file(args.to_link.unwrap());
    let avoid_words = read_words_from_file(args.to_avoid.unwrap());

    // Call the chat completion API endpoint
    let request_body = build_request_body_for_clues(link_words, avoid_words, &model_id);
    let clues = get_clues_from_api(&base_url, &api_key, request_body).await?;

    if clues.is_empty() {
        println!("The language model didn't return any useful clues.");
    } else {
        let output = clues.join("\n");
        println!("{}", output);
    }

    Ok(())
}
