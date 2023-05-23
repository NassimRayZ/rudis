use std::{
    collections::{BinaryHeap, HashMap},
    sync::{Condvar, Mutex},
    time::Instant,
};

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
    pub fn remove(&mut self, key: &str) {
        self.data.remove(key);
    }
}

#[derive(Eq, PartialEq)]
pub struct State {
    pub key: String,
    pub instant: Instant,
}

impl State {
    pub fn new(key: &str, instant: Instant) -> Self {
        Self {
            key: key.to_string(),
            instant,
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.instant.cmp(&self.instant)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
pub struct PriorityQueue {
    pub entries: Mutex<BinaryHeap<State>>,
    pub filled: Condvar,
}

impl PriorityQueue {
    pub fn new() -> Self {
        Self {
            entries: Mutex::new(BinaryHeap::new()),
            filled: Condvar::new(),
        }
    }
}
