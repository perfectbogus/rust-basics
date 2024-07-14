use std::io;
use rand::prelude::*;

fn main() {
    let random = thread_rng().gen_range(0..100);

    println!("I'm thinking of a number between 1 to 100...");
    loop {
        let mut buffer= String::new();
        println!("Guess the number:");
        io::stdin().read_line(&mut buffer);
        let number: i32 = buffer.trim().parse().unwrap();

        if number > random {
            println!("lower...")
        } else if number < random {
            println!("higher...")
        } else {
            println!("You're right");
            break;
        }
    }

}
