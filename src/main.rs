use std::fs;
use std::path::PathBuf;
use clap::Parser;

/// Mastermind - An LLM-powered CLI tool to help you be a better spymaster in Codenames
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// A list of words to link together - the words from your team
    #[arg(short, long, required = true, value_name = "FILE")]
    link: PathBuf,

    /// A list of words to avoid - opponent's words, neutral words, and the assassin word
    #[arg(short, long, required = true, value_name = "FILE")]
    avoid: PathBuf,
}

fn main() {
    let args = Args::parse();

    // Read and parse the link words file
    let link_words = match fs::read_to_string(&args.link) {
        Ok(link_lines) => link_lines
            .lines()
            .map(|line| line.trim().to_string())
            .collect::<Vec<String>>(),
        Err(_) => {
            eprintln!("Failed to read link words file: {:?}", args.link);
            std::process::exit(1);
        }
    };

    // Read and parse the avoid words file
    let avoid_words = match fs::read_to_string(&args.avoid) {
        Ok(avoid_lines) => avoid_lines
            .lines()
            .map(|line| line.trim().to_string())
            .collect::<Vec<String>>(),
        Err(_) => {
            eprintln!("Failed to read avoid words file: {:?}", args.avoid);
            std::process::exit(1);
        }
    };


    // Testing
    eprintln!("Link words: {:?}", link_words);
    eprintln!("Avoid words: {:?}", avoid_words);
}