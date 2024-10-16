use std::ops::Add;
use chrono::{NaiveDate, Utc};

enum Product {
    Book {
        isbn: String,
        title: String,
        price: f64,
    },
    Electronic {
        name: String,
        price: f64,
        warranty_months: u32,
    },
    Clothing {
        name: String,
        size: String,
        price: f64,
    },
    Food {
        name: String,
        expiry_date: String,
        price: f64,
    }
}

struct Inventory {
    products: Vec<Product>,
}

impl Inventory {
    fn new() -> Self {
        Inventory {
            products: Vec::new()
        }
    }

    fn add_product(&mut self, product: Product) {
        self.products.push(product);
    }

    fn total_value(&self) -> f64 {
        self.products.iter().map(|product| match product {
            Product::Book { price,.. } => *price,
            Product::Electronic { price,.. } => *price,
            Product::Clothing { price,.. } => *price,
            Product::Food { price,.. } => *price,
        }).sum()
    }

    fn apply_discount(&mut self, discount: f64) {
        self.products.iter_mut().for_each(|product| {
            let price = match product {
                Product::Book { price, .. } => price,
                Product::Electronic { price, .. } => price,
                Product::Clothing { price, .. } => price,
                Product::Food { price, .. } => price,
            };
            *price *= 1.0 - discount;
        })
    }

    fn list_expiring_food(&self, days: u32) -> Vec<String> {
        let today = Utc::now().date_naive();
        let expiry_threshold = today + chrono::Duration::days(days as i64);

        self.products
            .iter()
            .filter_map(|product| {
                if let Product::Food { name, expiry_date,..} = product {
                    if let Ok(expiry) = NaiveDate::parse_from_str(expiry_date,
                                                                  "%Y-%m-%d") {
                        if expiry <= expiry_threshold {
                            return Some(name.clone());
                        }
                    }
                }
                None
            }).collect()
    }

    fn find_product(&self, query: &str) -> Option<&Product> {
        self.products.iter().find(|&product| {
           match product {
               Product::Book { isbn, title, .. } =>
                   isbn.contains(query) || title.contains(query),
               Product::Electronic { name, .. } => name.contains(query),
               Product::Clothing { name, size, .. } =>
                   name.contains(query) || size.contains(query),
               Product::Food { name, .. } => name.contains(query),
           }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_value() {
        let mut inventory = Inventory::new();
        inventory.add_product(Product::Book { isbn: "123".to_string(), title: "Rust Programming".to_string(), price: 30.0 });
        inventory.add_product(Product::Electronic { name: "Laptop".to_string(), price: 1000.0, warranty_months: 12 });
        assert_eq!(inventory.total_value(), 1030.0);
    }

    #[test]
    fn test_apply_discount() {
        let mut inventory = Inventory::new();
        inventory.add_product(Product::Clothing { name: "T-Shirt".to_string(), size: "M".to_string(), price: 20.0 });
        inventory.apply_discount(0.1);
        assert_eq!(inventory.total_value(), 18.0);
    }
    #[test]
    fn test_list_expiring_food() {
        let mut inventory = Inventory::new();
        inventory.add_product(Product::Food { name: "Apple".to_string(), expiry_date: "2023-06-01".to_string(), price: 1.0 });
        inventory.add_product(Product::Food { name: "Banana".to_string(), expiry_date: "2023-05-15".to_string(), price: 0.5 });
        let expiring = inventory.list_expiring_food(7); // Assume today is 2023-05-10
        assert_eq!(expiring, vec!["Apple".to_string(), "Banana".to_string()]);
    }

    #[test]
    fn test_find_product() {
        let mut inventory = Inventory::new();
        inventory.add_product(Product::Book { isbn: "123".to_string(), title: "Rust Programming".to_string(), price: 30.0 });
        let found = inventory.find_product("Rust");
        assert!(matches!(found, Some(Product::Book { title, .. }) if title == "Rust Programming"));
    }
}