pub(crate) struct Clue {
    pub(crate) clue_word: String,
    pub(crate) count: usize,
    pub(crate) linked_words: Vec<String>,
    pub(crate) source: String,
}

impl Clue {
    /// Create a new instance of `Clue` from a single line of clue out of the API response
    pub fn new(clue_line: &str, source: String) -> Option<Self> {
        let chunks: Vec<String> = clue_line.split(",").map(|s| s.trim().to_string()).collect();

        // Discard empty lines as well as clues with only one word linked
        if chunks.len() < 4 {
            return None;
        }

        let clue_word = chunks[0].to_string();

        let Ok(count) = chunks[1].parse::<usize>() else {
            return None;
        };

        let linked_words: Vec<String> = chunks[2..].to_vec();

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ref_clue = Clue {
            clue_word: "gear".to_string(),
            count: 2,
            linked_words: vec!["scuba diver".to_string(), "hospital".to_string()],
            source: String::from("some_source"),
        };

        let clue = Clue::new(
            "   gear   , 2    ,   scuba diver  , hospital   ",
            String::from("some_source"),
        )
        .unwrap();

        assert_eq!(clue.clue_word, ref_clue.clue_word);
        assert_eq!(clue.count, ref_clue.count);
        assert_eq!(clue.linked_words, ref_clue.linked_words);
        assert_eq!(clue.source, ref_clue.source);

        let clue = Clue::new(
            "   gear   , 3    ,   scuba diver  , hospital   ",
            String::from("some_source"),
        );
        assert!(clue.is_none());

        let clue = Clue::new(
            "   gear   , a    ,   scuba diver  , hospital   ",
            String::from("some_source"),
        );
        assert!(clue.is_none());

        let clue = Clue::new(
            "   ge#ar   , 2    ,   scuba diver  , hospital   ",
            String::from("some_source"),
        );
        assert!(clue.is_none());

        let clue = Clue::new(
            "   gear   , 2    ,   scuba di/ver  , hospital   ",
            String::from("some_source"),
        );
        assert!(clue.is_none());
    }
}
