use std::fs;
use std::path::PathBuf;
use clap::Parser;

/// Mastermind - An LLM-powered CLI tool to help you be a better spymaster in Codenames
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    // TODO: Allow users to choose language models
    // /// A list of words to avoid - opponent's words, neutral words, and the assassin word
    // #[arg(short, long)]
    // model: Option<String>,

    /// Path to a file containing words to link together - the words from your team
    to_link: PathBuf,

    /// Path to a file containing words to avoid - opponent's words, neutral words, and the assassin word
    to_avoid: PathBuf,
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

fn main() {
    let args = Args::parse();

    let link_words = read_words_from_file(args.to_link);
    let avoid_words = read_words_from_file(args.to_avoid);

    let prompt = generate_prompt(link_words, avoid_words);

    // Testing
    println!("{}", prompt);
}