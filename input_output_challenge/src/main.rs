use std::env;
use std::fs;
use std::io::read_to_string;

fn main() {
    if env::args().len() <= 2 {
        println!("Program Requires at least 2 Arguments: Path ");
    }

    for (index, arg) in env::args().enumerate() {
        println!("Argument[{index}] = {arg}");
    }

    let path_file = env::args().nth(1).unwrap();
    println!("path_file is {path_file}");

    let name_to_search = env::args().nth(2).unwrap();
    println!("name to search: {name_to_search}");

    let content = read_to_string(&path_file)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();



}
