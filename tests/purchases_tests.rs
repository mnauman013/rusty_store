use rusty_store::inventory::Inventory;
use rusty_store::purchases::Purchases;

#[test]
fn test_record_purchase() {
    let mut inventory = Inventory::new();
    let product_id = inventory.add_product("Test Product".to_string(), "Test Description".to_string(), 10.0, 5);

    let mut purchases = Purchases::new();
    assert!(purchases.record_purchase(&mut inventory, product_id, 3, 8.0).is_ok());

    let product = inventory.get_product(product_id).unwrap();
    assert_eq!(product.quantity, 8); // Initial quantity (5) + purchased quantity (3)
}

#[test]
fn test_record_purchase_invalid_product() {
    let mut inventory = Inventory::new();
    let mut purchases = Purchases::new();
    assert!(purchases.record_purchase(&mut inventory, 999, 3, 8.0).is_err()); // Invalid product ID
}





