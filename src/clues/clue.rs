pub(crate) struct Clue {
    pub(crate) clue_word: String,
    pub(crate) count: usize,
    pub(crate) linked_words: Vec<String>,
    pub(crate) source: String,
}

impl Clue {
    /// Create a new instance of `Clue` from a single line of clue out of the API response
    pub fn new(clue_line: &str, source: String) -> Option<Self> {
        let chunks: Vec<&str> = clue_line.split(", ").collect();

        // Discard empty lines as well as clues with only one word linked
        if chunks.len() < 4 {
            return None;
        }

        let clue_word = chunks[0].trim().to_string();

        let Ok(count) = chunks[1].parse::<usize>() else {
            return None;
        };

        let linked_words: Vec<String> = chunks[2..].iter().map(|&s| s.trim().to_string()).collect();

        // Discard clue if count and the actual number don't line up
        if linked_words.len() != count {
            return None;
        }

        // Discard clues that contains special characters (likely due to hallucination)
        if !clue_word.chars().all(|c| c.is_alphabetic() || c == ' ') {
            return None;
        }
        for word in &linked_words {
            if !word.chars().all(|c| c.is_alphabetic() || c == ' ') {
                return None;
            }
        }

        Some(Self {
            clue_word,
            count,
            linked_words,
            source,
        })
    }
}
