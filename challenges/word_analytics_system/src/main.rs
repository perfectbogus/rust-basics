use std::collections::HashMap;

struct WordAnalytics {
    // Word frequency counter
    word_count: HashMap<String, usize>,
    // Word co-occurrence (which words appear together)
    word_pairs: HashMap<(String, String), usize>,
    // First occurrences of words (word -> position)
    first_seen: HashMap<String, usize>,
    // Word lengths distribution
    length_dist: HashMap<usize, Vec<String>>
}

impl WordAnalytics {
    fn new() -> Self {
        WordAnalytics {
            word_count: HashMap::new(),
            word_pairs: HashMap::new(),
            first_seen: HashMap::new(),
            length_dist: HashMap::new()
        }
    }

    fn add_text(&mut self, text: &str) {
        // TODO: Process text and update all hashmaps
        // Count words, track pairs, record first positions
        self.count_words(text);
        self.track_pairs(text);
        self.first_occurrence(text);

    }

    fn first_occurrence(&mut self, text: &str) {
        text.split_whitespace().enumerate()
            .for_each(|(pos, word)| {
                self.first_seen.entry(word.to_string())
                    .or_insert(pos);
            })
    }

    fn track_pairs(&mut self, text: &str) {
        let words: Vec<_> = text.split_whitespace().collect();
        words.windows(2)
            .for_each(|pair| {
                self.word_pairs
                    .entry((pair[0].to_string(), pair[1].to_string()))
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            })
    }

    fn word_distribution(&mut self, text: &str) {
        text.split_whitespace().for_each(|word| {
            let word_len = word.len();
            self.length_dist.entry(word_len)
                .or_insert_with(Vec::new)
                .push(word.to_string());
        })
    }

    fn count_words(&mut self, text: &str) {
        text.split_whitespace().into_iter()
            .for_each(|word| {
                self.word_count.entry(word.to_string())
                    .and_modify(|counter| *counter += 1 ).or_insert(1);
            })
    }

    fn most_frequent_words(&self, n: usize) -> Vec<(&String, &usize)> {
        // TODO: Return n most frequent words and their counts
        unimplemented!()
    }

    fn words_of_length(&self, length: usize) -> Option<&Vec<String>> {
        // TODO: Return all words of given length
        unimplemented!()
    }

    fn get_word_pairs(&self, word: &str) -> HashMap<&String, &usize> {
        // TODO: Return all words that appear with given word and their counts
        unimplemented!()
    }

    fn merge(&mut self, other: WordAnalytics) {
        // TODO: Merge two WordAnalytics instances
        // Combine counts, update first occurrences if earlier
        unimplemented!()
    }

    fn remove_word(&mut self, word: &str) {
        // TODO: Remove word from all HashMaps
        unimplemented!()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_text() {
        let mut analytics = WordAnalytics::new();
        analytics.add_text("hello world hello");

        // Test word count
        assert_eq!(*analytics.word_count.get("hello").unwrap(), 2);
        assert_eq!(*analytics.word_count.get("world").unwrap(), 1);

        // Test first occurrence
        assert_eq!(*analytics.first_seen.get("hello").unwrap(), 0);
        assert_eq!(*analytics.first_seen.get("world").unwrap(), 1);
    }
}

fn main() {
    println!("Hello, world!");

    let mut a = HashMap::new();

    for word in "this is a phrase that is short".split_whitespace() {
        a.entry(word.to_string()).and_modify(|e| *e += 1).or_insert(1);
    }

    println!("{:?}", a);

    let mut b: HashMap<String, usize> = HashMap::new();
    for word in "this another phrase".split_whitespace() {
        b.entry(word.to_string());
    }
    println!("{:?}", b);

    let mut c: HashMap<(String, String), usize> = HashMap::new();

    let mut buffer_word = "";
    for word in "this is a new phrase".split_whitespace() {
        c.entry((buffer_word.to_string(), word.to_string())).and_modify(|e| *e += 1).or_insert(1);
        buffer_word = word;
    }
    println!("{:?}", c);


}
