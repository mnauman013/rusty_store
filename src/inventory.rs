use crate::models::Product;
use std::collections::HashMap;

pub struct Inventory {
    products: HashMap<u32, Product>,
    next_product_id: u32,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory {
            products: HashMap::new(),
            next_product_id: 1,
        }
    }

    pub fn add_product(&mut self, name: String, description: String, price: f64, quantity: u32) -> u32 {
        let id = self.next_product_id;
        self.next_product_id += 1;
        self.products.insert(id, Product { id, name, description, price, quantity });
        id
    }

    pub fn edit_product(&mut self, id: u32, name: Option<String>, description: Option<String>, price: Option<f64>, quantity: Option<u32>) -> Result<(), String> {
        if let Some(product) = self.products.get_mut(&id) {
            if let Some(name) = name {
                product.name = name;
            }
            if let Some(description) = description {
                product.description = description;
            }
            if let Some(price) = price {
                product.price = price;
            }
            if let Some(quantity) = quantity {
                product.quantity = quantity;
            }
            Ok(())
        } else {
            Err("Product not found".to_string())
        }
    }

    pub fn delete_product(&mut self, id: u32) -> Result<(), String> {
        if self.products.remove(&id).is_some() {
            Ok(())
        } else {
            Err("Product not found".to_string())
        }
    }

    pub fn get_product(&self, id: u32) -> Option<&Product> {
        self.products.get(&id)
    }

    pub fn get_products(&self) -> Vec<&Product> {
        self.products.values().collect()
    }
}
