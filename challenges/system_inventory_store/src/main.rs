use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Game {
    title: String,
    genre: String,
    price: f64,
    stock: u32,
}

struct GameStore {
    inventory: HashMap<String, Game>,
}

impl Game {
    fn new(title: String, genre: String, price: f64) -> Self {
        Game {title, genre, price, stock: 0}
    }
}

impl GameStore {
    fn new() -> Self {
        Self { inventory: HashMap::new() }
    }

    //Insert using different methods
    fn add_game_simple(&mut self, game: Game) -> Option<Game> {
        //TODO: Use simple insert method
        unimplemented!()
    }
}

fn main() {
    println!("Hello, world!");
}
