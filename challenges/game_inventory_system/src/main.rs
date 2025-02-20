use std::collections::{HashMap, HashSet};
use std::fmt::format;

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
        for (ingredient, quantity) in &ingredients {
            if let Some(current_quantity) = self.prices.get(ingredient) {
                if *quantity > *current_quantity {
                    return Err(format!("{} is less than the recipe quantity", ingredient));
                }
            }
        }
        self.recipes.insert(result, ingredients);
        Ok(())
    }

    // Medium: Try to craft item
    fn craft(&mut self, item: &str) -> Result<(), String> {
        // Check recipe exists and have ingredients
        let recipe = self.recipes.get(item).ok_or("Recipe not found")?;

        for (ingredient, required_amount) in recipe {
            let available = self.items.get(ingredient)
                .ok_or(format!("Missing ingredient: {}", ingredient))?;

            if available < required_amount {
                return Err(format!("Not enough {}: have {} need {}", ingredient, available, required_amount));
            }
        }

        for (ingredient, amount) in recipe {
            *self.items.get_mut(ingredient).unwrap();
        }
        *self.items.entry(item.to_string()).or_insert(0) += 1;
        Ok(())
    }

    // Hard: Get craftable items with current inventory
    fn get_craftable(&self) -> Vec<String> {
        // let mut craftable: Vec<String> = Vec::new();
        // // Return items that can be crafted with current inventory
        // for (recipe_name, recipe) in &self.recipes {
        //     let mut valid_recipe: bool = true;
        //     println!("recipe: {}", recipe_name);
        //     for (ingredient, required_amount) in recipe {
        //         let available = self.items.get(ingredient).unwrap();
        //         println!("ingredient: {}, amount: {}, available: {}", ingredient, required_amount, available);
        //         if available < required_amount {
        //             valid_recipe = false;
        //             break;
        //         }
        //     }
        //     if valid_recipe {
        //         craftable.push(recipe_name.to_string());
        //     }
        // }
        // craftable
        self.recipes.iter()
            .filter(|(_, recipe)| {
                recipe.iter().all(|(ingredient, required_amount)| {
                    self.items.get(ingredient)
                        .map_or(false, |available| available >= required_amount)
                })
            })
            .map(|(item, _)| item.clone())
            .collect()
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
        ingredients.insert("wood".to_string(), 2);
        ingredients.insert("iron".to_string(), 1);

        inv.prices.insert("wood".to_string(), 5);
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

    #[test]
    fn test_craft_sequence() {
        let mut inv = GameInventory::new();

        // Add base materials
        inv.add_item("wood".to_string(), 4);
        inv.add_item("iron".to_string(), 3);

        // Add recipes
        let mut axe_recipe = HashMap::new();
        axe_recipe.insert("wood".to_string(), 2);
        axe_recipe.insert("iron".to_string(), 1);
        inv.add_recipe("axe".to_string(), axe_recipe).unwrap();

        let mut sword_recipe = HashMap::new();
        sword_recipe.insert("axe".to_string(), 1); // Sword requires an axe
        sword_recipe.insert("iron".to_string(), 2);
        inv.add_recipe("sword".to_string(), sword_recipe).unwrap();

        // Test crafting sequence for sword
        let sequence = inv.craft_sequence("sword", 1).unwrap();
        assert!(sequence, vec!["axe", "sword"]);

        assert!(inv.craft_sequence("diamond_sword", 1).is_err());
        assert!(inv.craft_sequence("sword", 2).is_err());
    }

}