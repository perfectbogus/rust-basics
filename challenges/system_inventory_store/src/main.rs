use std::collections::hash_map::Entry;
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
        self.inventory.insert(game.title.clone(), game)
    }

    fn add_game_entry(&mut self, game: Game) -> &mut Game{
        //TODO: Use entry API to insert and return mutable reference
        self.inventory.entry(game.title.clone()).or_insert(game)
    }

    fn add_or_update_stock(&mut self, title: &str, quantity: u32) -> Result<u32, String> {
        // TODO: Use entry API to add game or update stock if exists
        // Return new stock quantity
        match self.inventory.entry(title.to_string()) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().stock += quantity;
                Ok(entry.get().stock)
            },
            Entry::Vacant(_) => {
                Err("Game not found".to_string())
            }
        }
    }

    fn get_game_simple(&self, title: &str) -> Option<&Game>{
        // TODO: Use simple get method
        self.inventory.get(title)
    }

    fn get_game_or_default(&self, title: &str) -> Game {
        // TODO: Return the game or a default game with "Not Found" title
        self.inventory
            .get(title)
            .cloned()
            .unwrap_or_else(||
                Game::new("Not Found".to_string(), "Not Found".to_string(), 0.0))
    }

    fn get_price_if_in_stock(&self, title: &str) -> Option<f64> {
        // TODO: Return price only if game exists and has stock
        self.inventory
            .get(title)
            .filter(|game| game.stock > 0)
            .map(|game| game.price)
    }

    fn try_get_mut(&mut self, title: &str) -> Result<&mut Game, String> {
        // TODO: Try to get mutable reference or return error
        self.inventory
            .get_mut(title)
            .ok_or_else(|| "Not Found".to_string())
    }

}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_game() -> Game {
        Game::new(
            "Test Game".to_string(),
            "Action".to_string(),
            59.99,
        )
    }

    #[test]
    fn test_add_game_simple() {
        let mut store = GameStore::new();
        let game = create_test_game();

        // First insertion should return None
        assert!(store.add_game_simple(game.clone()).is_none());
        // Second insertion should return the old game
        let old_game = store.add_game_simple(game.clone());
        assert!(old_game.is_some());
        assert_eq!(old_game.unwrap().title, "Test Game");
    }

    #[test]
    fn test_add_game_entry() {
        let mut store = GameStore::new();
        let game = create_test_game();

        // Get mutable reference to inserted game
        let game_ref = store.add_game_entry(game.clone());
        assert_eq!(game_ref.title, "Test Game");

        // Modify the game through the reference
        game_ref.stock = 10;
        assert_eq!(store.inventory.get("Test Game").unwrap().stock, 10);
    }

    #[test]
    fn test_add_or_update_stock() {
        let mut store = GameStore::new();
        let game = create_test_game();
        store.add_game_simple(game);

        // Update existing game
        let new_stock = store.add_or_update_stock("Test Game", 5).unwrap();
        assert_eq!(new_stock, 5);

        // Update again
        let new_stock = store.add_or_update_stock("Test Game", 3).unwrap();
        assert_eq!(new_stock, 8);

        // Try to update non-existent game
        assert!(store.add_or_update_stock("Not Found", 5).is_err());
    }

    #[test]
    fn test_get_methods() {
        let mut store = GameStore::new();
        let game = create_test_game();
        store.add_game_simple(game);

        // Test simple get
        assert!(store.get_game_simple("Test Game").is_some());
        assert!(store.get_game_simple("Not Found").is_none());

        // Test get or default
        let default_game = store.get_game_or_default("Not Found");
        assert_eq!(default_game.title, "Not Found");

        // Test get price if in stock
        assert!(store.get_price_if_in_stock("Test Game").is_none()); // No stock
        store.add_or_update_stock("Test Game", 1).unwrap();
        assert_eq!(store.get_price_if_in_stock("Test Game").unwrap(), 59.99);

        // Test try get mut
        assert!(store.try_get_mut("Test Game").is_ok());
        assert!(store.try_get_mut("Not Found").is_err());
    }
}