use std::collections::HashMap;
use std::hash::Hash;

pub struct Counter<T: Eq + Hash> {
    values: HashMap<T, u64>,
}

impl<T: Eq + Hash> Counter<T> {
    pub fn new() -> Self {
        Counter {
            values: HashMap::new(),
        }
    }

    pub fn count(&mut self, value: T) {
        *self.values.entry(value).or_insert(0) += 1;
    }

    pub fn times_seen(&self, value: &T) -> u64 {
        self.values.get(value).copied().unwrap_or_default()
    }
}