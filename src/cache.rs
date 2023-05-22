use std::collections::HashMap;

#[derive(Default)]
pub struct Cache {
    data: HashMap<String, String>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }
    pub fn get(&self, key: String) -> Option<&String> {
        self.data.get(&key)
    }
}
