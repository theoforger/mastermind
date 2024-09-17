use clap::Parser;
use std::io;
use std::path::PathBuf;

pub mod api;
mod clue;

/// Mastermind - An LLM-powered CLI tool to help you be a better spymaster in Codenames
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Get available language json_models from API
    #[arg(short, long = "get-models")]
    pub get: bool,

    /// Specify a language model
    #[arg(short, long = "set-model")]
    pub model: Option<String>,

    /// Path to a file containing words to link together - the words from your team
    #[arg(required_unless_present = "get")]
    pub to_link: Option<PathBuf>,

    /// Path to a file containing words to avoid - opponent's words, neutral words, and the assassin word
    #[arg(required_unless_present = "get")]
    pub to_avoid: Option<PathBuf>,
}

pub fn read_words_from_file(path: PathBuf) -> Result<Vec<String>, io::Error> {
    let contents = std::fs::read_to_string(&path).map_err(|_| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("Cannot find file: {}", path.to_string_lossy()),
        )
    })?;

    let words: Vec<String> = contents
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();

    if words.is_empty() {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("File is empty: {}", path.to_string_lossy()),
        ))
    } else {
        Ok(words)
    }
}
