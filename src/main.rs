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

fn main() {
    let args = Args::parse();

    // Read and parse the link words file
    let link_words = match fs::read_to_string(&args.to_link) {
        Ok(link_lines) => link_lines
            .lines()
            .map(|line| line.trim().to_string())
            .collect::<Vec<String>>(),
        Err(_) => {
            eprintln!("Failed to read link words file: {:?}", args.to_link);
            std::process::exit(1);
        }
    };

    // Read and parse the avoid words file
    let avoid_words = match fs::read_to_string(&args.to_avoid) {
        Ok(avoid_lines) => avoid_lines
            .lines()
            .map(|line| line.trim().to_string())
            .collect::<Vec<String>>(),
        Err(_) => {
            eprintln!("Failed to read avoid words file: {:?}", args.to_avoid);
            std::process::exit(1);
        }
    };


    // Constructing prompt
    let mut prompt = String::from("To Link:\n");
    link_words.iter().for_each(|word| {
        prompt.push_str(&word);
        prompt.push_str("\n");
    });
    prompt.push_str("\nTo Avoid:\n");
    avoid_words.iter().for_each(|word| {
        prompt.push_str(&word);
        prompt.push_str("\n");
    });

    // Testing
    eprintln!("{}", prompt);
}