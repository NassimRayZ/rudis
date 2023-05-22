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
    pub fn set(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
    }
    pub fn get(&self, key: &str) -> Option<String> {
        self.data.get(key).map(|s| s.to_owned())
    }
}
