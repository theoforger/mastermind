use std::fs;
use std::io;
use std::path::PathBuf;

pub mod api_handlers;
mod clue;

pub fn read_words_from_file(path: PathBuf) -> Result<Vec<String>, io::Error> {
    let contents = fs::read_to_string(&path).map_err(|_| {
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
