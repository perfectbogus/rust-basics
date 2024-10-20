struct ShoppingList {
    items: Vec<String>
}

impl ShoppingList {
    fn new() -> Self {
        ShoppingList {
            items: Vec::new()
        }
    }

    fn add_item(&mut self, item: String) {
        self.items.push(item)
    }

    fn remove_item(&mut self, item: &str) -> bool {
        if let Some(index) = self.items
            .iter()
            .position(|r| r == item) {
            self.items.swap_remove(index);
            return true;
        }
        false
    }

    fn print_list(&self) {
        self.items.iter().for_each(|item| println!("item: {}", item))
    }

    fn sort_list(&mut self) {
        self.items.sort();
    }

    fn total_items(&self) -> usize {
        self.items.len()
    }

    fn clear_list(&mut self) {
        self.items.clear();
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_remove() {
        let mut list = ShoppingList::new();
        list.add_item("Apples".to_string());
        assert_eq!(list.total_items(), 1);
        assert!(list.remove_item("Apples"));
        assert_eq!(list.total_items(), 0);
    }

    #[test]
    fn test_sort() {
        let mut list = ShoppingList::new();
        list.add_item("Milk".to_string());
        list.add_item("Apples".to_string());
        list.add_item("Bread".to_string());
        list.sort_list();
        assert_eq!(list.items, vec!["Apples", "Bread", "Milk"]);
    }


    #[test]
    fn test_clear() {
        let mut list = ShoppingList::new();
        list.add_item("Apples".to_string());
        list.add_item("Milk".to_string());
        list.clear_list();
        assert_eq!(list.total_items(), 0);
    }
}