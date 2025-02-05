use std::collections::HashMap;
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
                        re.unwrap().is_match(&c.text)
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

    fn count_words(&self, content_id: u32) -> Result<usize, String> {
        // TODO: Count words based on content type:
        // - PlainText: Split by whitespace
        // - Markdown: Ignore markdown symbols (#, *, _, etc)
        // - HTML: Ignore HTML tags
        // - Code: Count only non-keyword words
        self.contents.iter()
            .find(|c| c.id == content_id)
            .map_or_else(
                || Err(format!("Content {} does not exists", content_id)),
                |c| match &c.content_type {
                    ContentType::PlainText => {
                        Ok(c.text.split_whitespace().count())
                    },
                    ContentType::Markdown => {
                        let cleaned = c.text
                            .lines()
                            .filter(|line| !line.starts_with("#"))
                            .map(|line| line
                                .replace("*", "")
                                .replace("_", "")
                                .replace("`", "")
                            )
                            .collect::<Vec<String>>()
                            .join(" ");

                        Ok(cleaned.split_whitespace().count())
                    },
                    ContentType::Html => {
                        let cleaned = c.text
                            .replace(regex::Regex::new(r"<[^>]*>").unwrap().as_str(), " ");
                        
                        Ok(cleaned.split_whitespace().count())
                    },
                    ContentType::Code(lang) => {
                        let keywords = match lang.as_str() {
                            "rust" => vec!["fn", "let", "mut", "struct"],
                            "python" => vec!["def", "class", "import", "from", "return"],
                            _ => vec![]
                        };

                        Ok(c.text
                            .split_whitespace()
                            .filter(|word| !keywords.contains(&word))
                            .count()
                        )
                    }
                }
            )
    }

    fn categorize(&self) -> HashMap<String, Vec<&Content>> {
        // TODO: Categorize content by type:
        // - "text" for PlainText
        // - "markup" for Markdown and HTML
        // - "code" for Code (with language)
        self.contents.iter().fold(HashMap::new(), |mut map, content| {
            let category = match &content.content_type {
                ContentType::PlainText => { "text" }
                ContentType::Markdown | ContentType::Html => { "markup" }
                ContentType::Code(_) => { "code" }
            };

            map.entry(category.to_string())
                .or_insert_with(Vec::new)
                .push(content);

            map
        })
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

#[cfg(test)]
mod tests {
    use crate::ContentType::{Code, Html, Markdown, PlainText};
    use super::*;

    fn create_contents() -> Vec<Content> {
        let tuple = vec![
            (PlainText, "Hello World".to_string()),
            (Html, "<h1>Hi</h1>".to_string()),
            (Markdown, "<h1>Hi</h1>".to_string()),
            (Code("rust".to_string()), "fn main() {}".to_string())
        ];

        tuple.into_iter()
            .enumerate()
            .map(|(index, (content_type, text))| {
                Content {
                    id: index as u32,
                    content_type,
                    text,
                    metadata: None,
                }
            }).collect()
    }

    #[test]
    fn test_find_by_type() {
        let mut processor = TextProcessor::new();

        let contents = create_contents();
        //contents.into_iter().for_each(|content| { processor.add_content(content); });
        for content in contents {
            processor.add_content(content);
        }

        // Test exact match for Code type
        let rust_content = processor.find_by_type(
            &ContentType::Code("rust".to_string())
        );
        assert_eq!(rust_content.len(), 1);

        // Test no match for different language
        let python_content = processor.find_by_type(
            &ContentType::Code("python".to_string())
        );
        assert_eq!(python_content.len(), 0);
    }

    #[test]
    fn test_find_by_pattern() {
        let mut processor = TextProcessor::new();

        let content = Content {
            id: 1,
            content_type: ContentType::PlainText,
            text: "Hello World".to_string(),
            metadata: None,
        };
        processor.add_content(content);

        // Test Simple contains patter
        let pattern = TextPattern::Contains("Hello".to_string());
        assert_eq!(processor.find_by_pattern(&pattern).len(), 1);

        // Test AND Pattern
        let pattern = TextPattern::And(
            Box::new(TextPattern::Contains("Hello".to_string())),
            Box::new(TextPattern::Contains("World".to_string()))
        );
        assert_eq!(processor.find_by_pattern(&pattern).len(), 1);
    }

    #[test]
    fn test_count_words() {
        let mut processor = TextProcessor::new();

        let content = Content {
            id: 1,
            content_type: ContentType::PlainText,
            text: "Hello World".to_string(),
            metadata: None,
        };
        processor.add_content(content);

        assert_eq!(processor.count_words(1).unwrap(), 2);
    }

    #[test]
    fn test_categorize() {
        let mut processor = TextProcessor::new();

        let content = Content {
            id: 1,
            content_type: ContentType::Code("rust".to_string()),
            text: "fn main() {}".to_string(),
            metadata: None,
        };
        processor.add_content(content);

        let categories = processor.categorize();
        assert!(categories.contains_key("code"));
        assert_eq!(categories["code"].len(), 1);
    }

    #[test]
    fn test_pattern_starts_with() {
        let mut processor = TextProcessor::new();

        let contents = create_contents();
        for content in contents {
            processor.add_content(content);
        }

        let pattern = TextPattern::StartsWith("Hello".to_string());
        let results = processor.find_by_pattern(&pattern);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, 0);
    }

    #[test]
    fn test_pattern_ends_with() {
        let mut processor = TextProcessor::new();

        create_contents().into_iter().for_each(|content| { processor.add_content(content); });

        let pattern = TextPattern::EndsWith("World".to_string());
        let results = processor.find_by_pattern(&pattern);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, 0);
    }

    #[test]
    fn test_pattern_regex() {

    }
}

fn main() {
    println!("Hello, world!");
}
