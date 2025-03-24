#[derive(Debug, Clone)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub quantity: u32,
}

#[derive(Debug)]
pub struct Sale {
    pub id: u32,
    pub product_id: u32,
    pub quantity_sold: u32,
    pub sale_price: f64,
}
#[derive(Debug)]
pub struct Purchase {
    pub id: u32,
    pub product_id: u32,
    pub quantity_purchased: u32,
    pub purchase_price: f64,
}