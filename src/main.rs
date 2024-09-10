use clap::Parser;
use dotenv::dotenv;
use serde_json::json;
use std::path::PathBuf;
use std::{env, fs};

mod json_models;
use json_models::chat_completion::*;
use json_models::language_models::*;

/// Mastermind - An LLM-powered CLI tool to help you be a better spymaster in Codenames
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    // TODO
    /// Specify a language model
    #[arg(short, long)]
    model: Option<String>,

    /// Get all available language json_models from API
    #[arg(short, long)]
    get: bool,

    /// Path to a file containing words to link together - the words from your team
    #[arg(required_unless_present = "get")]
    to_link: Option<PathBuf>,

    /// Path to a file containing words to avoid - opponent's words, neutral words, and the assassin word
    #[arg(required_unless_present = "get")]
    to_avoid: Option<PathBuf>,
}

fn read_words_from_file(path: PathBuf) -> Vec<String> {
    match fs::read_to_string(&path) {
        Ok(contents) => contents
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .collect::<Vec<String>>(),
        Err(e) => {
            eprintln!("Failed to read file at {:?}: {}", path, e);
            std::process::exit(1);
        }
    }
}

fn generate_prompt(link_words: Vec<String>, avoid_words: Vec<String>) -> String {
    format!(
        "To Link:\n{}\n\nTo Avoid:\n{}",
        link_words.join("\n"),
        avoid_words.join("\n")
    )
}

fn build_request_body(prompt: String) -> serde_json::Value {
    let system_prompt = "I am the spymaster in Codenames. I will give you a list of agent words to link together, followed by a list of agent words to avoid. Give me a list of clue words followed by the agent words they are supposed to link together. With each clue word, try to link as many agent words as possible
                    Here are the requirements:
                    - Always answer in lower case
                    - Give five clue word options
                    - Never give any intro, outro or explanation
                    - Answer in this format:
                        [clue] [number of agent words] - [agent word] [agent word] [agent word]
                    - For the agent words, only give the words themselves. Do not add anything.";
    json!({
        "messages": [
            {
                "role": "system",
                "content": system_prompt,
            },
            {
                "role": "user",
                "content": prompt
            }
        ],
        "model": "llama-3.1-70b-versatile"
    })
}

async fn get_clues_from_api(
    endpoint: String,
    key: &str,
    body: serde_json::Value,
) -> reqwest::Result<String> {
    let client = reqwest::Client::new();
    let response = client
        .post(endpoint)
        .bearer_auth(key)
        .json(&body)
        .send()
        .await?;

    Ok(response.json::<ChatCompletionResponse>().await?.choices[0]
        .message
        .content
        .to_owned())
}

async fn get_model_ids_from_api(endpoint: String, key: &str) -> reqwest::Result<String> {
    let client = reqwest::Client::new();
    let response = client.get(endpoint).bearer_auth(key).send().await?;

    Ok(response
        .json::<ModelsResponse>()
        .await?
        .data
        .iter()
        .map(|model| model.id.as_str())
        .collect::<Vec<&str>>()
        .join("\n"))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read arguments
    let args = Args::parse();

    // API Configuration
    dotenv().ok();
    let api_key = env::var("API_KEY")?;
    let mut base_url = env::var("OPENAI_API_BASE_URL")?;
    if !base_url.ends_with('/') {
        base_url.push('/');
    }

    // if -g is set, call the json_models API instead
    if args.get {
        let models_endpoint = format!("{}models", base_url);
        println!(
            "{}",
            get_model_ids_from_api(models_endpoint, &api_key).await?
        );
        return Ok(());
    }

    // Get the prompt ready
    let link_words = read_words_from_file(args.to_link.unwrap());
    let avoid_words = read_words_from_file(args.to_avoid.unwrap());
    let prompt = generate_prompt(link_words, avoid_words);

    // Call the chat completion API
    let chat_completion_endpoint = format!("{}chat/completions", base_url);
    let body = build_request_body(prompt);
    let clues = get_clues_from_api(chat_completion_endpoint, &api_key, body).await?;

    println!("{}", clues);
    Ok(())
}
