fn main() {
    let s1;
    let s = String::from("lifetime challenge");  // This WILL be dropped!

    {
        let ta = TextAnalyzer { text: &s };
        s1 = ta.text;
    }

    println!("text: {}", s1);
}

struct TextAnalyzer<'a> {
    text: &'a str,
}