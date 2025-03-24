use crate::models::Sale;
use crate::inventory::Inventory;
use crate::error::StoreError;

pub struct Sales {
    sales: Vec<Sale>,
    next_sale_id: u32,
}

impl Sales {
    pub fn new() -> Self {
        Sales {
            sales: Vec::new(),
            next_sale_id: 1,
        }
    }

    pub fn record_sale(&mut self, inventory: &mut Inventory, product_id: u32, quantity_sold: u32, sale_price: f64) -> Result<(), StoreError> {
        if let Some(product) = inventory.get_product(product_id) {
            if product.quantity >= quantity_sold {
                let sale = Sale {
                    id: self.next_sale_id,
                    product_id,
                    quantity_sold,
                    sale_price,
                };
                self.sales.push(sale);
                self.next_sale_id += 1;
                inventory.edit_product(product_id, None, None, None, Some(product.quantity - quantity_sold))?;
                Ok(())
            } else {
                Err(StoreError::InsufficientStock)
            }
        } else {
            Err(StoreError::ProductNotFound)
        }
    }

    pub fn get_sales(&self) -> &[Sale] {
        &self.sales
    }
}
