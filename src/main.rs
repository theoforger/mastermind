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
    /// Get available language json_models from API
    #[arg(short, long = "get-models")]
    get: bool,

    // TODO
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

fn build_request_body(prompt: String, model_id: String) -> serde_json::Value {
    let system_prompt = "You are the spymaster in Codenames.
                    I will give you a list of words to link together, followed by a list of words to avoid.
                    Respond with a list of clue words followed by the words they are supposed to link together.
                    With each clue word, try to link as many words as possible.
                    Here are the requirements:
                    - Always answer in lower case
                    - Give 5 to 10 clue word options
                    - Never give any intro, outro or explanation
                    - Only give the words themselves. Do not add anything else
                    - Answer in this format:
                        [clue] [number of agent words] - [agent word] [agent word] [agent word]
                    ";
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
        "model": model_id
    })
}

async fn get_clues_from_api(
    endpoint: String,
    key: &str,
    body: serde_json::Value,
) -> reqwest::Result<Vec<String>> {
    let client = reqwest::Client::new();
    let response = client
        .post(endpoint)
        .bearer_auth(key)
        .json(&body)
        .send()
        .await?;

    let mut clues = response.json::<ChatCompletionResponse>().await?.choices[0]
        .message
        .content
        .lines()
        .map(|line| line.trim().to_string())
        .collect::<Vec<String>>();

    cleanup_clues(&mut clues);

    Ok(clues)
}

async fn get_model_ids_from_api(endpoint: String, key: &str) -> reqwest::Result<Vec<String>> {
    let client = reqwest::Client::new();
    let response = client.get(endpoint).bearer_auth(key).send().await?;

    let mut model_ids = response
        .json::<ModelsResponse>()
        .await?
        .data
        .iter()
        .map(|model| model.id.trim().to_string())
        .collect::<Vec<String>>();
    model_ids.sort();

    Ok(model_ids)
}

// Remove possible LLM hallucination
fn cleanup_clues(clues: &mut Vec<String>) {
    clues.retain(|clue| {
        let words: Vec<&str> = clue.split_whitespace().collect();
        if words.len() > 4 {
            if let Ok(count) = words[1].parse::<usize>() {
                if words.len() == count + 3 {
                    return true;
                }
            }
        }

        false
    });

    clues.sort_by(|a, b| {
        let a_words: Vec<&str> = a.split_whitespace().collect();
        let b_words: Vec<&str> = b.split_whitespace().collect();
        b_words[1].cmp(&a_words[1])
    });
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
        let models_endpoint = format!("{}models", base_url);
        let output = get_model_ids_from_api(models_endpoint, &api_key)
            .await?
            .join("\n");
        println!("{}", output);
        return Ok(());
    }

    // If -m is set, use a preferred language model. Otherwise, use the default
    let model_id = if args.model.is_some() {
        args.model.unwrap()
    } else {
        env::var("DEFAULT_MODEL_ID")?
    };

    // Ready the prompt
    let link_words = read_words_from_file(args.to_link.unwrap());
    let avoid_words = read_words_from_file(args.to_avoid.unwrap());
    let prompt = generate_prompt(link_words, avoid_words);

    // Call the chat completion API endpoint
    let chat_completion_endpoint = format!("{}chat/completions", base_url);
    let body = build_request_body(prompt, model_id);
    let clues = get_clues_from_api(chat_completion_endpoint, &api_key, body).await?;

    if clues.is_empty() {
        println!("The language model didn't return any useful clues.");
    } else {
        let output = clues.join("\n");
        println!("{}", output);
    }

    Ok(())
}
