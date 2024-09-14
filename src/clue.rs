pub struct Clue {
    pub clue_word: String,
    pub count: usize,
    pub linked_words: Vec<String>,
}

pub struct ClueCollection {
    pub clues: Vec<Clue>,
}

impl Clue {
    /// Create a new instance of `Clue` from a single line of clue responses from the API
    pub fn new(clue_line: &str) -> Option<Self> {
        let chunks: Vec<&str> = clue_line.split_whitespace().collect();
        println!("{:?}", chunks);

        // Discard empty lines as well as clues with only one word linked
        if chunks.len() < 5 {
            return None;
        }

        let clue_word = chunks[0].to_string();
        let count = match chunks[1].parse::<usize>() {
            Ok(count) => count,
            Err(_) => return None,
        };
        let linked_words: Vec<String> = chunks[2..].iter().map(|&s| s.to_string()).collect();

        // Verify the clue
        if linked_words.len() != count {
            return None;
        }

        Some(Self {
            clue_word,
            count,
            linked_words,
        })
    }

    pub fn display(&self) {
        // TODO
    }
}

impl ClueCollection {
    /// Create an instance of `ClueCollection` from `Vec<String>`, which contains lines of clue response from the API
    pub fn new(clue_strings: Vec<String>) -> Self {
        Self {
            clues: clue_strings.iter().filter_map(|s| Clue::new(s)).collect(),
        }
    }

    /// Add `Clue` into the collection
    pub fn add(&mut self, clue: Clue) {
        self.clues.push(clue);
    }

    /// Sort the collection by number of words the clues link together
    pub fn sort(&mut self) {
        self.clues.sort_by(|a, b| a.count.cmp(&b.count));
    }

    pub fn is_empty(&self) -> bool {
        self.clues.is_empty()
    }

    pub fn display(&self) {
        // Todo
    }
}
