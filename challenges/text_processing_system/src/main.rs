use std::cmp::Ordering;
use regex::Regex;

#[derive(Debug, PartialEq)]
enum ContentType {
    PlainText,
    Markdown,
    Html,
    Code(String), // Programming language
}

#[derive(Debug, PartialEq)]
struct Content {
    id: u32,
    content_type: ContentType,
    text: String,
    metadata: Option<Metadata>,
}

#[derive(Debug, PartialEq)]
struct Metadata {
    author: String,
    tags: Vec<String>,
    created_at: u64 // timestamp
}

struct TextProcessor {
    contents: Vec<Content>,
}

impl TextProcessor {
    fn new() -> Self {
        Self { contents: Vec::new() }
    }

    fn add_content(&mut self, content: Content) {
        self.contents.push(content);
    }

    fn find_by_type(&self, content_type: &ContentType) -> Vec<&Content> {
        // TODO: Find all content of specific type
        // For Code type, match the exact language
        self.contents.iter()
            .filter(|c| {
                match (&c.content_type, content_type) {
                    (ContentType::Code(lang1), ContentType::Code(lang2)) =>
                        { lang1 == lang2 },
                    _ => c.content_type == *content_type
                }
            })
            .collect()
    }

    fn find_by_pattern(&self, pattern: &TextPattern) -> Vec<&Content> {
        // TODO: Find content matching the pattern
        self.contents.iter()
            .filter(|c| {
                match pattern {
                    TextPattern::Contains(s) => { c.text.contains(s) }
                    TextPattern::StartsWith(s) => { c.text.starts_with(s) }
                    TextPattern::EndsWith(s) => { c.text.ends_with(s) }
                    TextPattern::Regex(s) => {
                        let re = Regex::new(s).map_err(|_| "Invalid Regex");
                        re.is_match(&c.text)
                    }
                    TextPattern::And(left, right) => {
                        self.matches_pattern(c, left) && self.matches_pattern(c, right)
                    }
                    TextPattern::Or(left, right) => {
                        self.matches_pattern(c, left) || self.matches_pattern(c, right)
                    }
                    TextPattern::Not(tp) => {
                        !self.matches_pattern(c, tp)
                    }
                }
            })
            .collect()
    }

    fn matches_pattern(&self, content: &Content, pattern: &TextPattern) -> bool {
        self.find_by_pattern(pattern).contains(&content)
    }
}

#[derive(Debug)]
enum TextPattern {
    Contains(String),
    StartsWith(String),
    EndsWith(String),
    Regex(String),
    And(Box<TextPattern>, Box<TextPattern>),
    Or(Box<TextPattern>, Box<TextPattern>),
    Not(Box<TextPattern>),
}












fn main() {
    println!("Hello, world!");
}
