use crate::models::Purchase;
use crate::inventory::Inventory;
use crate::error::StoreError;

pub struct Purchases {
    purchases: Vec<Purchase>,
    next_purchase_id: u32,
}

impl Purchases {
    pub fn new() -> Self {
        Purchases {
            purchases: Vec::new(),
            next_purchase_id: 1,
        }
    }

    pub fn record_purchase(&mut self, inventory: &mut Inventory, product_id: u32, quantity_purchased: u32, purchase_price: f64) -> Result<(), StoreError> {
        if let Some(product) = inventory.get_product(product_id) {
            let new_quantity = product.quantity + quantity_purchased;
            inventory.edit_product(product_id, None, None, None, Some(new_quantity))?;

            let purchase = Purchase {
                id: self.next_purchase_id,
                product_id,
                quantity_purchased,
                purchase_price,
            };
            self.purchases.push(purchase);
            self.next_purchase_id += 1;
            Ok(())
        } else {
            Err(StoreError::ProductNotFound)
        }
    }

    pub fn get_purchases(&self) -> &[Purchase] {
        &self.purchases
    }
}
