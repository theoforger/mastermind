use clap::Parser;
use std::fs;
use std::path::PathBuf;

pub mod api;
pub mod clues;
pub mod configs;
pub mod model_collection;

mod json;

/// Mastermind - An LLM-powered CLI tool to help you be a better spymaster in Codenames
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Print all available language models
    #[arg(short, long = "get-models")]
    pub get: bool,

    /// Select language model(s)
    #[arg(short, long = "set-models", default_missing_value = "interactive", num_args = 0..)]
    pub models: Option<Vec<String>>,

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

pub fn read_words_from_file(path: &PathBuf) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)
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
    path: &PathBuf,
    content: String,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(existing_content) = fs::read_to_string(path) {
        if !existing_content.is_empty() {
            return Err(format!("File is not empty: {}", path.to_string_lossy()).into());
        }
    }

    fs::write(path, content)
        .map_err(|_| format!("Failed to write to file: {}", path.to_string_lossy()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_read_words_from_file() {
        let to_link = read_words_from_file(&PathBuf::from("examples/link.txt"));
        assert!(to_link.is_ok());
        let to_avoid = read_words_from_file(&PathBuf::from("examples/avoid.txt"));
        assert!(to_avoid.is_ok());
    }

    #[test]
    fn test_write_content_to_file() {
        // Invalid path
        let write_result = write_content_to_file(
            &PathBuf::from("/none/existent/path/lol"),
            String::from("useless text"),
        );
        assert!(write_result.is_err());

        // Successful write
        let temp_dir = tempdir().unwrap();
        let temp_file = temp_dir.path().join("temp.txt");
        write_content_to_file(&temp_file, String::from("some text")).unwrap();
        assert_eq!(fs::read_to_string(&temp_file).unwrap(), "some text");

        // Non-empty file
        let write_result = write_content_to_file(&temp_file, String::from("some text again"));
        assert!(write_result.is_err());
    }
}
