use std::ops::Index;

struct Item {
    id: String,
    name: String,
    quantity: usize,
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
        
        Ok(())
    }
    
}

fn main() {
    println!("Hello, world!");
}
