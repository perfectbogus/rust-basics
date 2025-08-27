fn main() {
    let mut message = String::from("Hello rust");

    // This function will modify the string
    make_uppercase(&mut message);

    println!("after uppercase: {}", message);
}

fn make_uppercase(s: &mut String) {
    *s = s.to_uppercase();
}

