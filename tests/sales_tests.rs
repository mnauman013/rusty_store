use rusty_store::inventory::Inventory;
use rusty_store::sales::Sales;

#[test]
fn test_record_sale() {
    let mut inventory = Inventory::new();
    let product_id = inventory.add_product("Test Product".to_string(), "Test Description".to_string(), 10.0, 5);

    let mut sales = Sales::new();
    assert!(sales.record_sale(&mut inventory, product_id, 2, 10.0).is_ok());

    let product = inventory.get_product(product_id).unwrap();
    assert_eq!(product.quantity, 3); // Initial quantity (5) - sold quantity (2)
}

#[test]
fn test_record_sale_insufficient_stock() {
    let mut inventory = Inventory::new();
    let product_id = inventory.add_product("Test Product".to_string(), "Test Description".to_string(), 10.0, 5);

    let mut sales = Sales::new();
    assert!(sales.record_sale(&mut inventory, product_id, 10, 10.0).is_err()); // Not enough stock
}

#[test]
fn test_record_sale_invalid_product() {
    let mut inventory = Inventory::new();
    let mut sales = Sales::new();
    assert!(sales.record_sale(&mut inventory, 999, 2, 10.0).is_err()); // Invalid product ID
}
