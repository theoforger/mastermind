use std::fs;
use std::path::PathBuf;

pub mod api_handlers;
mod json_models;

pub fn read_words_from_file(path: PathBuf) -> Vec<String> {
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
