use std::fmt::{Display, Formatter, Result, write};

struct Password(String);

impl Display for Password {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "**********")
    }
}

fn main() {
    let unsecure_pass = "unsecure_pass".to_string();
    let secure_pass = Password(unsecure_pass.clone());
    println!("secure: {secure_pass}");
    println!("unsecure: {unsecure_pass}");
}