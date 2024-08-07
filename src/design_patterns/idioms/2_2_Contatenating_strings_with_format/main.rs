fn say_hello(name: &str) -> String {
    format!("hello {}", name)
}
fn main() {
    let hello_string = say_hello("Bruno");
    println!("{}", hello_string);
}