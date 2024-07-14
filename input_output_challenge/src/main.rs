use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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

    if let Ok(lines) = read_lines(path_file) {
        for line in lines.flatten() {
            if line == name_to_search {
                println!("{name_to_search} did walk on the Moon");
                return;
            }
        }
    }

    println!("{name_to_search} did not walk on the moon")
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
