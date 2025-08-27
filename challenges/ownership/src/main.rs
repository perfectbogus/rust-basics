fn main() {
    let message = String::from("Hello rust");

    let length = get_string_length(&message);

    println!("Length: {}", length);

    //try to uncomment this line and see what happens;
    println!("Original message: {}", message);
    let _len = get_length(&message);
    println!("Length: {}", _len);
    let _contains = contains_rust(&message);
    println!("Contains rust: {}", _contains );
}

fn get_string_length(s: &String) -> usize {
    s.len()
}

fn get_length(s: &str) -> usize {
    s.len()
}

fn contains_rust(s: &str) -> bool {
    let to_search = String::from("rust");
    s.contains(&to_search)
}

