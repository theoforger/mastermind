use std::path::PathBuf;
use clap::Parser;

/// Mastermind - An LLM-powered CLI tool to help you be a better spymaster in Codenames
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// FILE should contain a list of words to link together - the words from your team
    #[arg(short, long, value_name = "FILE")]
    link: Option<PathBuf>,

    /// FILE should contain a list of words to avoid - opponent's words, neutral words, and the assassin word
    #[arg(short, long, value_name = "FILE")]
    avoid: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}