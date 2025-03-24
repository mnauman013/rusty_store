use crate::models::{Product, Sale, Purchase};

pub fn generate_inventory_report(inventory: &[&Product]) {
    println!("=== Inventory Report ===");
    for product in inventory {
        println!(
            "ID: {}, Name: {}, Quantity: {}, Price: ${:.2}",
            product.id, product.name, product.quantity, product.price
        );
    }
}

pub fn generate_sales_report(sales: &[Sale]) {
    println!("=== Sales Report ===");
    for sale in sales {
        println!(
            "Sale ID: {}, Product ID: {}, Quantity Sold: {}, Sale Price: ${:.2}",
            sale.id, sale.product_id, sale.quantity_sold, sale.sale_price
        );
    }
}

pub fn generate_purchases_report(purchases: &[Purchase]) {
    println!("=== Purchases Report ===");
    for purchase in purchases {
        println!(
            "Purchase ID: {}, Product ID: {}, Quantity Purchased: {}, Purchase Price: ${:.2}",
            purchase.id, purchase.product_id, purchase.quantity_purchased, purchase.purchase_price);
        }
}
