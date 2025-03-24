use std::collections::HashMap;

pub struct Auth {
    users: HashMap<String, String>,
}

impl Auth {
    pub fn new() -> Self {
        Auth {
            users: HashMap::new(),
        }
    }

    pub fn add_user(&mut self, username: String, password: String) {
        self.users.insert(username, password);
    }

    pub fn authenticate(&self, username: &str, password: &str) -> bool {
        self.users.get(username).map_or(false, |p| p == password)
    }
}