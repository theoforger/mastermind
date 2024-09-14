use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, CellAlignment, ContentArrangement, Table};

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

        // Discard empty lines as well as clues with only one word linked
        if chunks.len() < 4 {
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
        self.clues.sort_by(|a, b| b.count.cmp(&a.count));
    }

    pub fn is_empty(&self) -> bool {
        self.clues.is_empty()
    }

    pub fn display(&self) {
        let mut table = Table::new();
        
        // Set up header and styles
        table
            .set_header(vec![
                Cell::new("Clue")
                    .add_attribute(Attribute::Bold)
                    .set_alignment(CellAlignment::Center),
                Cell::new("Count")
                    .add_attribute(Attribute::Bold)
                    .set_alignment(CellAlignment::Center),
                Cell::new("Linked Words")
                    .add_attribute(Attribute::Bold)
                    .set_alignment(CellAlignment::Center),
            ])
            .set_content_arrangement(ContentArrangement::Dynamic)
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS);

        // Add rows
        for clue in &self.clues {
            table.add_row(vec![
                clue.clue_word.clone(),
                clue.count.to_string(),
                clue.linked_words.join(" "),
            ]);
        }

        // Center the second column
        let second_column = table.column_mut(1).expect("The table has three columns");
        second_column.set_cell_alignment(CellAlignment::Center);

        println!("{table}");
    }
}
