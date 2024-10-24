use std::collections::HashMap;

struct WordCounter {
    word_frequencies: HashMap<String, u32>,
}

impl WordCounter {
    fn new() -> Self {
        Self {
            word_frequencies: HashMap::new()
        }
    }

    fn add_text(&mut self, text: &str) {
        // TODO: Add words from text to the counter
        // Hint: Split the text into words and count each one
        text.split_whitespace()
            .for_each(|word| {
                self.word_frequencies.entry(word.to_string())
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            });
    }

    fn get_frequency(&self, word: &str) -> u32 {
        // TODO: Get frequency of a specific word
        // Return 0 if word is not found
        self.word_frequencies.get(word).copied().unwrap_or(0)
    }

    fn get_most_common(&self) -> Option<(&String, &u32)> {
        self.word_frequencies
            .iter()
            .max_by_key(|(_, count)| *count)
    }

    fn get_words_with_frequency(&self, frequency: u32) -> Vec<&String> {
        self.word_frequencies.iter()
            .filter(|(_, &count)| count == frequency )
            .map(|(word, _)| word )
            .collect()
    }

    fn clear_word(&mut self, word: &str) {
        self.word_frequencies.remove_entry(word);
    }


}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_get_frequency() {
        let mut counter = WordCounter::new();
        counter.add_text("hello world hello");
        assert_eq!(counter.get_frequency("hello"), 2);
        assert_eq!(counter.get_frequency("world"), 1);
        assert_eq!(counter.get_frequency("notfound"), 0);
    }

    #[test]
    fn test_most_common() {
        let mut counter = WordCounter::new();
        counter.add_text("hello world hello rust world hello");
        let (word, count) = counter.get_most_common().unwrap();
        assert_eq!(word, "hello");
        assert_eq!(*count, 3);
    }

    #[test]
    fn test_words_with_frequency() {
        let mut counter = WordCounter::new();
        counter.add_text("hello world hello rust world hello rust");
        let words = counter.get_words_with_frequency(2);
        assert_eq!(words.len(), 2);
        assert!(words.contains(&&String::from("rust")));
    }

    #[test]
    fn test_clear_word() {
        let mut counter = WordCounter::new();
        counter.add_text("hello world hello");
        counter.clear_word("hello");
        assert_eq!(counter.get_frequency("hello"), 0);
        assert_eq!(counter.get_frequency("world"), 1);
    }
}