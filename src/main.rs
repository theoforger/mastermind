use clap::Parser;
use dotenv::dotenv;
use serde::Deserialize;
use serde_json::json;
use std::path::PathBuf;
use std::{env, fs};

/// Mastermind - An LLM-powered CLI tool to help you be a better spymaster in Codenames
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    // TODO: Allow users to choose language models
    /// Specify your preferred language model
    #[arg(short, long)]
    model: Option<String>,

    /// Path to a file containing words to link together - the words from your team
    to_link: PathBuf,

    /// Path to a file containing words to avoid - opponent's words, neutral words, and the assassin word
    to_avoid: PathBuf,
}

#[derive(Deserialize)]
struct GorqResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Deserialize)]
struct Message {
    content: String,
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

async fn get_answer_from_api_endpoint(
    endpoint: String,
    key: String,
    body: serde_json::Value,
) -> reqwest::Result<String> {
    let client = reqwest::Client::new();
    let response = client
        .post(endpoint)
        .bearer_auth(key)
        .json(&body)
        .send()
        .await?;

    Ok(response.json::<GorqResponse>().await?.choices[0]
        .message
        .content
        .to_owned())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let link_words = read_words_from_file(args.to_link);
    let avoid_words = read_words_from_file(args.to_avoid);

    let prompt = generate_prompt(link_words, avoid_words);

    // API
    dotenv().ok();
    let api_key = env::var("GROQ_API_KEY")?;
    let endpoint = env::var("GROQ_API_ENDPOINT")?;

    let body = build_request_body(prompt);
    let answer = get_answer_from_api_endpoint(endpoint, api_key, body).await?;

    println!("{}", answer);
    Ok(())
}
