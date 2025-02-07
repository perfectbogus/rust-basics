use std::collections::{HashMap, HashSet};
fn main() {
    println!("Hello, world!");
}

// A game inventory system with crafting and trading
struct GameInventory {
    items: HashMap<String, u32>,
    recipes: HashMap<String, HashMap<String, u32>>,
    prices: HashMap<String, u32>,
    bank: u32,
}

impl GameInventory {
    fn new () -> Self {
        GameInventory {
            items: HashMap::new(),
            recipes: HashMap::new(),
            prices: HashMap::new(),
            bank: 100, // Starting Money
        }
    }

    fn add_item(&mut self, item: String, quantity: u32) {
        *self.items.entry(item).or_insert(0) += quantity;
    }

    // Medium: Add crafting recipe
    fn add_recipe(&mut self, result: String, ingredients: HashMap<String, u32>) -> Result<(), String> {
        // Validate ingredients exist in prices
        unimplemented!()
    }

    // Medium: Try to craft item
    fn craft(&mut self, item: &str) -> Result<(), String> {
        // Check recipe exists and have ingredients
        unimplemented!()
    }

    // Hard: Get craftable items with current inventory
    fn get_craftable(&self) -> Vec<String> {
        // Return items that can be crafted with current inventory
        unimplemented!()
    }

    // Hard: Calculate optimal crafting sequence
    fn craft_sequence(&self, item: &str, quantity: u32) -> Result<Vec<String>, String> {
        // Return sequence of items to craft to reach goal
        unimplemented!()
    }

    // Expert: Trade optimization
    fn optimize_trades(&self, goal_item: &str, quantity: u32) -> Option<Vec<(String, u32)>> {
        // Return optimal sequence of trades to acquire goal item
        // Consider crafting costs vs buying directly
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_item() {
        let mut inv = GameInventory::new();
        inv.add_item("wood".to_string(), 5);
        inv.add_item("wood".to_string(), 3);
        assert_eq!(*inv.items.get("wood").unwrap(), 8);
    }

    #[test]
    fn test_recipe() {
        let mut inv = GameInventory::new();

        let mut ingredients = HashMap::new();
        ingredients.insert("wood".to_string(), 5);
        ingredients.insert("iron".to_string(), 1);

        inv.prices.insert("wood".to_strin(), 5);
        inv.prices.insert("iron".to_string(), 10);

        assert!(inv.add_recipe("axe".to_string(), ingredients).is_ok());
    }

    #[test]
    fn test_craft() {
        let mut inv = GameInventory::new();

        inv.add_item("wood".to_string(), 4);
        inv.add_item("iron".to_string(), 2);

        let mut recipe = HashMap::new();
        recipe.insert("wood".to_string(), 2);
        recipe.insert("iron".to_string(), 1);

        inv.add_recipe("axe".to_string(), recipe).unwrap();

        assert!(inv.craft("axe").is_ok());
        assert_eq!(*inv.items.get("wood").unwrap(), 2);
        assert_eq!(*inv.items.get("iron").unwrap(), 1);
        assert_eq!(*inv.items.get("axe").unwrap(), 1);
    }

    #[test]
    fn test_get_craftable() {
        let mut inv = GameInventory::new();

        inv.add_item("wood".to_string(), 2);
        inv.add_item("iron".to_string(), 1);

        let mut axe_recipe = HashMap::new();
        axe_recipe.insert("wood".to_string(), 2);
        axe_recipe.insert("iron".to_string(), 1);
        inv.add_recipe("axe".to_string(), axe_recipe).unwrap();

        let mut sword_recipe = HashMap::new();
        sword_recipe.insert("wood".to_string(), 1);
        sword_recipe.insert("iron".to_string(), 2);
        inv.add_recipe("sword".to_string(), sword_recipe).unwrap();

        let craftable = inv.get_craftable();
        assert!(craftable.contains(&"axe".to_string()));
        assert!(!craftable.contains(&"sword".to_string()));
    }

}