use rusty_store::inventory::Inventory;
use rusty_store::models::Product;

#[test]
fn test_add_product() {
    let mut inventory = Inventory::new();
    let id = inventory.add_product("Test Product".to_string(), "Test Description".to_string(), 10.0, 5);
    assert_eq!(inventory.get_product(id).unwrap().name, "Test Product");
}

#[test]
fn test_edit_product() {
    let mut inventory = Inventory::new();
    let id = inventory.add_product("Test Product".to_string(), "Test Description".to_string(), 10.0, 5);
    assert!(inventory.edit_product(id, Some("Updated Name".to_string()), None, None, None).is_ok());
    assert_eq!(inventory.get_product(id).unwrap().name, "Updated Name");
}

#[test]
fn test_delete_product() {
    let mut inventory = Inventory::new();
    let id = inventory.add_product("Test Product".to_string(), "Test Description".to_string(), 10.0, 5);
    assert!(inventory.delete_product(id).is_ok());
    assert!(inventory.get_product(id).is_none());
}

#[test]
fn test_delete_nonexistent_product() {
    let mut inventory = Inventory::new();
    assert!(inventory.delete_product(999).is_err()); // Invalid product ID
}
