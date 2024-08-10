
// Borrowing the owned type &String
fn three_vowels_borrowing(word: &String) -> bool {
    let mut vowel_count = 0;
    for c in word.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowel_count += 1;
                if vowel_count >= 3 {
                    return true
                }
            },
            _ => vowel_count = 0
        }
    }
    false
}

//borrowed type -> &str
fn three_vowels_borrowed(word: &str) -> bool {
    let mut vowel_count = 0;
    for c in word.chars() {
        match c {
            'a' | 'e' | 'i' | 'o'| 'u' => {
                vowel_count += 1;
                if vowel_count >= 3 {
                    return true
                }
            },
            _ => vowel_count = 0
        }
    }
    false
}


fn main() {
    let ferris_string: String = "Ferris".to_string();
    let curious_string: String = "Curious".to_string();

    let ferris_str: &str = "Ferris";
    let curious_str: &str = "Curious";

    //Borrowing the owner type
    println!("{}: {}", ferris_string, three_vowels_borrowing(&ferris_string));
    println!("{}: {}", curious_string, three_vowels_borrowing(&curious_string));
    // Fail
    // println!("{}: {}", ferris_string, three_vowels_borrowing(ferris_str));
    // println!("{}: {}", curious_string, three_vowels_borrowing(&curious_str));

    //Borrowed the type
    println!("{}: {}", ferris_string, three_vowels_borrowed(ferris_str));
    println!("{}: {}", curious_string, three_vowels_borrowed(curious_str));

    let sentence_string =
        "Once upon a time, there was a friendly curious crab named Ferris".to_string();

    for word in sentence_string.split(' ') {
        if three_vowels_borrowed(word) {
            println!("{word} has three consecutive vowels!");
        }
    }

}