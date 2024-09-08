use std::fs;
use std::path::PathBuf;
use clap::Parser;

/// Mastermind - An LLM-powered CLI tool to help you be a better spymaster in Codenames
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// FILE should contain a list of words to link together - the words from your team
    #[arg(short, long, required = true, value_name = "FILE")]
    link: PathBuf,

    /// FILE should contain a list of words to avoid - opponent's words, neutral words, and the assassin word
    #[arg(short, long, required = true, value_name = "FILE")]
    avoid: PathBuf,
}

fn main() {
    let args = Args::parse();
    
    let link_words: Vec<String>;
    let avoid_words: Vec<String>;

    // If the attempt to read both files succeeds, collect each line of the files into their corresponding vector
    if let (Ok(link_lines), Ok(avoid_lines)) = (fs::read_to_string(args.link), fs::read_to_string(args.avoid)) {
        link_words = link_lines
            .lines()
            .map(|line| line.trim().to_string())
            .collect();
        avoid_words = avoid_lines
            .lines()
            .map(|line| line.trim().to_string())
            .collect();
    } else {
        eprintln!("Failed to read file(s): Make sure your input path is correct.");
        std::process::exit(1);
    }


    // Testing
    eprintln!("Link words: {:?}", link_words);
    eprintln!("Avoid words: {:?}", avoid_words);
}