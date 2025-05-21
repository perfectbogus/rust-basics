use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() -> io::Result<()> {
    
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return Ok(());
    }

    let filename = &args[1];
    let path = Path::new(filename);
    
    if !path.exists() {
        println!("File not found: {}", filename);
        return Ok(());
    }
    
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    
    let mut word_counts: HashMap<String, usize> = HashMap::new();
    let mut total_words = 0;
    
    for line in reader.lines() {
        let line = line?;
        for word in line.split_whitespace() {
            let clean_word = word
                .trim_matches(|c: char| !c.is_alphanumeric())
                .to_lowercase();
            
            if !clean_word.is_empty() {
                *word_counts.entry(clean_word).or_insert(0) += 1;
                total_words += 1;
            }
        }
    }
    
    let mut word_counts_vec: Vec<(&String, &usize)> = word_counts
        .iter().collect();
    word_counts_vec.sort_by(|a, b| b.1.cmp(a.1));

    println!("Word Frequency Analysis for '{}':", filename);
    println!("Total words: {}", total_words);
    println!("Unique words: {}", word_counts.len());
    println!("\nTop 10 most common words:");
    
    for (i, (word, count)) in word_counts_vec.iter().take(10).enumerate() {
        let percentage = (**count as f64 / total_words as f64) * 100.0;
        println!("{}. '{}' - {} times ({:.2}%)",
                 i + 1,
                 word,
                 count,
                 percentage
        );
    }
    
    Ok(())
}
