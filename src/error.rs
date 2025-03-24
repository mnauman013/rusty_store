use std::fmt;

#[derive(Debug)]
pub enum StoreError {
    ProductNotFound,
    InsufficientStock,
    InvalidInput,
    Unauthorized,
    Other(String), // Add a variant to handle generic String errors
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StoreError::ProductNotFound => write!(f, "Product not found"),
            StoreError::InsufficientStock => write!(f, "Insufficient stock"),
            StoreError::InvalidInput => write!(f, "Invalid input"),
            StoreError::Unauthorized => write!(f, "Unauthorized access"),
            StoreError::Other(msg) => write!(f, "{}", msg), // Handle generic String errors
        }
    }
}

// Implement From<String> for StoreError
impl From<String> for StoreError {
    fn from(error: String) -> Self {
        StoreError::Other(error) // Convert String to StoreError::Other
        }
}
