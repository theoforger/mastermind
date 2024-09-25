use clap::Parser;
use std::fs;
use std::path::PathBuf;

pub mod api;
pub mod clue;
pub mod json_models;
#[cfg(test)]
mod tests;

/// Mastermind - An LLM-powered CLI tool to help you be a better spymaster in Codenames
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Print all available language models
    #[arg(short, long = "get-models")]
    pub get: bool,

    /// Select language model(s)
    #[arg(short, long = "set-model", num_args = 1..)]
    pub model: Option<Vec<String>>,

    /// Specify an output file
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,

    /// Print token usage information
    #[arg(short, long = "token-usage")]
    pub token: bool,

    /// File containing words to link together - the words from your team
    #[arg(required_unless_present = "get")]
    pub to_link: Option<PathBuf>,

    /// File containing words to avoid - opponent's words, neutral words, and the assassin word
    #[arg(required_unless_present = "get")]
    pub to_avoid: Option<PathBuf>,
}

pub fn read_words_from_file(path: PathBuf) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(&path)
        .map_err(|_| format!("Cannot find file: {}", path.to_string_lossy()))?;

    let words: Vec<String> = contents
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();

    if words.is_empty() {
        Err(format!("File is empty: {}", path.to_string_lossy()).into())
    } else {
        Ok(words)
    }
}

pub fn write_content_to_file(
    path: PathBuf,
    content: String,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(existing_content) = fs::read_to_string(&path) {
        if !existing_content.is_empty() {
            return Err(format!("File is not empty: {}", path.to_string_lossy()).into());
        }
    }

    fs::write(&path, content)
        .map_err(|_| format!("Failed to write to file: {}", path.to_string_lossy()))?;

    Ok(())
}
