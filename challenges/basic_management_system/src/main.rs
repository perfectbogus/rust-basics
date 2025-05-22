use std::ops::Index;

#[derive(Debug)]
struct Item {
    id: String,
    name: String,
    quantity: i32,
    price: f64,
    category: String,
}

struct Inventory {
    items: Vec<Item>
}

impl Inventory {
    
    fn new() -> Self {
        Inventory {
            items: Vec::new()
        }
    }

    fn len(&self) -> usize {
        self.items.len()
    }

    fn add_item(&mut self, item: Item) {
        self.items.push(item)
    }
    
    fn get_item(&self, id: &str) -> Option<&Item> {
        self.items.iter().find(|item| item.id == id)
    }
    
    fn get_item_mut(&mut self, id: &str) -> Option<&mut Item> {
        self.items.iter_mut().find(|item| item.id == id)
    }
    
    fn remove_item(&mut self, id: &str) -> Option<Item> {
        unimplemented!()
    }
    
    fn transfer_item(&mut self, dest: &mut Inventory, id: &str) -> Result<(), String> {
        if let Some(item) = self.remove_item(id) {
            dest.add_item(item);
            Ok(())
        } else {
            Err(format!("Item not found: {}", id))
        }
    }

    fn update_quantity(&mut self, id: &str, quantity: i32) -> Result<(), String> {
        if let Some(item) = self.get_item_mut(id) {
            item.quantity = quantity;
            Ok(())
        } else {
            Err(format!("Item not found: {}", id))
        }
    }

    fn list_low_stock(&self, threshold: i32) -> Vec<&Item> {
        self.items.iter().filter(|item| item.quantity <= threshold).collect()
    }

    fn calculate_value(&self) -> f64 {
        self.items.iter().map(|item| (item.quantity as f64) * item.price ).sum()
    }

    fn search_item(&self, category: &str) -> Vec<&Item> {
        self.items.iter().filter(|item| item.category == category).collect()
    }

}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::{Inventory, Item};

    fn create_item() -> Item {
        Item {
            id: "abc".to_string(),
            name: "soda".to_string(),
            quantity: 10,
            price: 2.5,
            category: "Misc".to_string(),
        }
    }

    #[test]
    fn add_item() {
        let mut inventory = Inventory::new();
        let new_item = create_item();

        assert_eq!(inventory.items.len(), 0);

        inventory.add_item(new_item);

        assert_eq!(inventory.len(), 1);
    }

    #[test]
    fn get_item() {
        let mut inventory = Inventory::new();
        let item = create_item();
        
        inventory.add_item(item);
        
        let opt = inventory.get_item(&"abc");
        
        assert_eq!(opt.unwrap().name, "soda".to_string());
        
    }
}