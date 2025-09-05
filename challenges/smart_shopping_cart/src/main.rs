// Easy Challenge: Smart Shopping Cart
// Focus: Basic Rc<RefCell<T>> patterns + simple algorithms

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

// TODO: Your implementation here!

#[derive(Debug, Clone)]
struct Item {
    name: String,
    price: f64,
}

// TODO: Define your ShoppingCart struct
// TODO: Define your Customer struct
struct ShoppingCart {
    items: RefCell<Vec<Item>>,
}

impl ShoppingCart {
    fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(ShoppingCart {
            items: RefCell::new(Vec::new()),
        }))
    }

    fn add_item(&self, item: Item) {
        self.items.borrow_mut().push(item);
    }
}

struct Customer {
    name: String,
    cart: Rc<RefCell<ShoppingCart>>,
    purchase_history: RefCell<Vec<Item>>,
}

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_cart_operations() {
        let cart = ShoppingCart::new();

        let apple = Item { name: "Apple".to_string(), price: 1.50 };
        let banana = Item { name: "Banana".to_string(), price: 0.75 };

        cart.add_item(apple.clone());
        cart.add_item(banana.clone());

        assert_eq!(cart.item_count(), 2);
        assert_eq!(cart.total_price(), 2.25);
    }

    #[test]
    fn test_shared_cart() {
        let cart = ShoppingCart::new();

        // Two customers share the same cart
        let customer1 = Customer::new("Alice".to_string(), cart.clone());
        let customer2 = Customer::new("Bob".to_string(), cart.clone());

        let apple = Item { name: "Apple".to_string(), price: 1.50 };
        let banana = Item { name: "Banana".to_string(), price: 0.75 };

        // Customer1 adds apple
        customer1.add_to_cart(apple);

        // Customer2 adds banana
        customer2.add_to_cart(banana);

        // Both should see both items
        assert_eq!(customer1.get_cart_total(), 2.25);
        assert_eq!(customer2.get_cart_total(), 2.25);
        assert_eq!(customer1.get_cart_item_count(), 2);
    }

    #[test]
    fn test_find_most_expensive_item() {
        let cart = ShoppingCart::new();

        cart.add_item(Item { name: "Cheap".to_string(), price: 1.0 });
        cart.add_item(Item { name: "Expensive".to_string(), price: 10.0 });
        cart.add_item(Item { name: "Medium".to_string(), price: 5.0 });

        let most_expensive = cart.find_most_expensive().unwrap();
        assert_eq!(most_expensive.name, "Expensive");
        assert_eq!(most_expensive.price, 10.0);
    }

    #[test]
    fn test_remove_items() {
        let cart = ShoppingCart::new();

        cart.add_item(Item { name: "Apple".to_string(), price: 1.50 });
        cart.add_item(Item { name: "Banana".to_string(), price: 0.75 });

        assert_eq!(cart.item_count(), 2);

        let removed = cart.remove_item("Apple");
        assert!(removed);
        assert_eq!(cart.item_count(), 1);
        assert_eq!(cart.total_price(), 0.75);

        let not_found = cart.remove_item("Orange");
        assert!(!not_found);
    }

    #[test]
    fn test_customer_purchase_history() {
        let cart = ShoppingCart::new();
        let customer = Customer::new("Alice".to_string(), cart.clone());

        customer.add_to_cart(Item { name: "Apple".to_string(), price: 1.50 });
        customer.add_to_cart(Item { name: "Banana".to_string(), price: 0.75 });

        // Customer "purchases" (moves items from cart to history)
        customer.purchase();

        // Cart should be empty
        assert_eq!(cart.item_count(), 0);

        // Customer should have purchase history
        assert_eq!(customer.purchase_history_count(), 2);
        assert_eq!(customer.total_spent(), 2.25);
    }

    #[test]
    fn test_multiple_customers_independence() {
        let cart1 = ShoppingCart::new();
        let cart2 = ShoppingCart::new();

        let customer1 = Customer::new("Alice".to_string(), cart1.clone());
        let customer2 = Customer::new("Bob".to_string(), cart2.clone());

        customer1.add_to_cart(Item { name: "Apple".to_string(), price: 1.50 });
        customer2.add_to_cart(Item { name: "Banana".to_string(), price: 0.75 });

        // Each customer should only see their own items
        assert_eq!(customer1.get_cart_item_count(), 1);
        assert_eq!(customer2.get_cart_item_count(), 1);
        assert_eq!(customer1.get_cart_total(), 1.50);
        assert_eq!(customer2.get_cart_total(), 0.75);
    }
}
