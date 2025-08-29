struct DocumentProcessor {
    documents: Vec<String>
}

impl DocumentProcessor {

    fn new() -> Self {
        DocumentProcessor {
            documents: Vec::new()
        }
    }

    fn add_document(&mut self, text: String) {
        self.documents.push(text);
    }

    fn find_longest_word(&self) -> String {
        let mut longest_word = "";
        for document in &self.documents {
            for word in document.split_whitespace() {
                if word.len() > longest_word.len() {
                    longest_word = word;
                }
            }
        }
        longest_word.to_string()
    }

    fn make_document_uppercase(&mut self, index: usize) {
        let upper = self.documents.get_mut(index).unwrap().to_uppercase();
        *self.documents.get_mut(index).unwrap() = upper;
    }

    fn count_containing_word(&self, word: &str) -> usize {
        self.documents.iter().filter(|&doc| doc.contains(word)).count()
    }
}

fn main() {
    let mut processor = DocumentProcessor::new();

    processor.add_document(String::from("Hello, world!"));
    processor.add_document(String::from("Hello, Rust"));
    processor.add_document(String::from("uppercase word"));

    let longest = processor.find_longest_word();
    println!("Longest: {}", longest);

    processor.make_document_uppercase(1);
    println!("Docs: {:?}", processor.documents);

    let count = processor.count_containing_word("Hello");
    println!("Count: {}", count);
}