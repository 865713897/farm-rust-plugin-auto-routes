use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone, Debug)]
pub struct Cache<K, V> {
    data: HashMap<K, V>,
}

impl<K: Eq + Hash + Clone, V> Cache<K, V> {
    pub fn new() -> Self {
        Cache {
            data: HashMap::new(),
        }
    }

    pub fn remove(&mut self, key: &K) {
        self.data.remove(key);
    }

    pub fn get_or_insert_with<F: FnOnce() -> V>(&mut self, key: &K, f: F) -> &V {
        self.data.entry(key.clone()).or_insert_with(f)
    }
}
