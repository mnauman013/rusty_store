mod inventory;
mod sales;
mod purchases;
mod reporting;
mod auth;
mod error;
mod models;

use std::io::{self, Write};
use inventory::Inventory;
use sales::Sales;
use purchases::Purchases;
use reporting::{generate_inventory_report, generate_sales_report, generate_purchases_report};
use auth::Auth;

fn main() {
    let mut inventory = Inventory::new();
    let mut sales = Sales::new();
    let mut purchases = Purchases::new();
    let mut auth = Auth::new();

    // Add a default user for testing
    auth.add_user("admin".to_string(), "password".to_string());

    // Main loop
    loop {
        println!("=== Rusty Store Inventory Management System ===");
        println!("1. Login");
        println!("2. Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = choice.trim().parse().unwrap_or(0);

        match choice {
            1 => {
                // Login
                println!("Enter username: ");
                let mut username = String::new();
                io::stdin().read_line(&mut username).unwrap();
                println!("Enter password: ");
                let mut password = String::new();
                io::stdin().read_line(&mut password).unwrap();

                if auth.authenticate(&username.trim(), &password.trim()) {
                    println!("Login successful!");
                    logged_in_menu(&mut inventory, &mut sales, &mut purchases);
                } else {
                    println!("Invalid credentials!");
                }
            }
            2 => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid option!"),
        }
    }
}

/// Menu for logged-in users
fn logged_in_menu(inventory: &mut Inventory, sales: &mut Sales, purchases: &mut Purchases) {
    loop {
        println!("=== Main Menu ===");
        println!("1. Manage Inventory");
        println!("2. Record Sale");
        println!("3. Record Purchase");
        println!("4. Generate Reports");
        println!("5. Logout");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = choice.trim().parse().unwrap_or(0);

        match choice {
            1 => inventory_menu(inventory),
            2 => record_sale(inventory, sales),
            3 => record_purchase(inventory, purchases),
            4 => generate_reports(inventory, sales, purchases),
            5 => {
                println!("Logging out...");
                break;
            }
            _ => println!("Invalid option!"),
        }
    }
}

/// Menu for managing inventory
fn inventory_menu(inventory: &mut Inventory) {
    loop {
        println!("=== Inventory Management ===");
        println!("1. Add Product");
        println!("2. Edit Product");
        println!("3. Delete Product");
        println!("4. View Inventory");
        println!("5. Back to Main Menu");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = choice.trim().parse().unwrap_or(0);

        match choice {
            1 => {
                println!("Enter product name: ");
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                println!("Enter product description: ");
                let mut description = String::new();
                io::stdin().read_line(&mut description).unwrap();
                println!("Enter product price: ");
                let mut price = String::new();
                io::stdin().read_line(&mut price).unwrap();
                let price: f64 = price.trim().parse().unwrap_or(0.0);
                println!("Enter product quantity: ");
                let mut quantity = String::new();
                io::stdin().read_line(&mut quantity).unwrap();
                let quantity: u32 = quantity.trim().parse().unwrap_or(0);

                let id = inventory.add_product(name.trim().to_string(), description.trim().to_string(), price, quantity);
                println!("Product added with ID: {}", id);
            }
            2 => {
                println!("Enter product ID: ");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                let id: u32 = id.trim().parse().unwrap_or(0);
                println!("Enter new product name (leave blank to skip): ");
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                println!("Enter new product description (leave blank to skip): ");
                let mut description = String::new();
                io::stdin().read_line(&mut description).unwrap();
                println!("Enter new product price (leave blank to skip): ");
                let mut price = String::new();
                io::stdin().read_line(&mut price).unwrap();
                let price: Option<f64> = if price.trim().is_empty() {
                    None
                } else {
                    Some(price.trim().parse().unwrap_or(0.0))
                };
                println!("Enter new product quantity (leave blank to skip): ");
                let mut quantity = String::new();
                io::stdin().read_line(&mut quantity).unwrap();
                let quantity: Option<u32> = if quantity.trim().is_empty() {
                    None
                } else {
                    Some(quantity.trim().parse().unwrap_or(0))
                };

                if let Err(e) = inventory.edit_product(id, Some(name.trim().to_string()), Some(description.trim().to_string()), price, quantity) {
                    println!("Error: {}", e);
                } else {
                    println!("Product updated successfully!");
                }
            }
            3 => {
                println!("Enter product ID: ");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                let id: u32 = id.trim().parse().unwrap_or(0);

                if let Err(e) = inventory.delete_product(id) {
                    println!("Error: {}", e);
                } else {
                    println!("Product deleted successfully!");
                }
            }
            4 => {
                generate_inventory_report(&inventory.get_products());
            }
            5 => break,
            _ => println!("Invalid option!"),
        }
    }
}

/// Record a sale
fn record_sale(inventory: &mut Inventory, sales: &mut Sales) {
    println!("Enter product ID: ");
    let mut product_id = String::new();
    io::stdin().read_line(&mut product_id).unwrap();
    let product_id: u32 = product_id.trim().parse().unwrap_or(0);
    println!("Enter quantity sold: ");
    let mut quantity_sold = String::new();
    io::stdin().read_line(&mut quantity_sold).unwrap();
    let quantity_sold: u32 = quantity_sold.trim().parse().unwrap_or(0);
    println!("Enter sale price: ");
    let mut sale_price = String::new();
    io::stdin().read_line(&mut sale_price).unwrap();
    let sale_price: f64 = sale_price.trim().parse().unwrap_or(0.0);

    if let Err(e) = sales.record_sale(inventory, product_id, quantity_sold, sale_price) {
        println!("Error: {}", e);
    } else {
        println!("Sale recorded successfully!");
    }
}

/// Record a purchase
fn record_purchase(inventory: &mut Inventory, purchases: &mut Purchases) {
    println!("Enter product ID: ");
    let mut product_id = String::new();
    io::stdin().read_line(&mut product_id).unwrap();
    let product_id: u32 = product_id.trim().parse().unwrap_or(0);
    println!("Enter quantity purchased: ");
    let mut quantity_purchased = String::new();
    io::stdin().read_line(&mut quantity_purchased).unwrap();
    let quantity_purchased: u32 = quantity_purchased.trim().parse().unwrap_or(0);
    println!("Enter purchase price: ");
    let mut purchase_price = String::new();
    io::stdin().read_line(&mut purchase_price).unwrap();
    let purchase_price: f64 = purchase_price.trim().parse().unwrap_or(0.0);

    if let Err(e) = purchases.record_purchase(inventory, product_id, quantity_purchased, purchase_price) {
        println!("Error: {}", e);
    } else {
        println!("Purchase recorded successfully!");
    }
}

/// Generate reports
fn generate_reports(inventory: &Inventory, sales: &Sales, purchases: &Purchases) {
    println!("=== Reports ===");
    generate_inventory_report(&inventory.get_products());
    generate_sales_report(sales.get_sales());
    generate_purchases_report(purchases.get_purchases());
}
