use std::collections::HashMap;

pub struct NoZeroHashMap {
    hashmap: HashMap<char, u32>,
}

impl NoZeroHashMap {
    pub fn new() -> Self {
        Self {
            hashmap: HashMap::new(),
        }
    }

    pub fn inc(&mut self, key: char) {
        if let Some(amount) = self.hashmap.get_mut(&key) {
            *amount += 1;
        } else {
            self.hashmap.insert(key, 1);
        }
    }

    pub fn dec(&mut self, key: char) {
        if let Some(amount) = self.hashmap.get_mut(&key) {
            if *amount == 1 {
                self.hashmap.remove(&key);
            } else {
                *amount -= 1;
            }
        }
    }

    pub fn len(&self) -> usize {
        return self.hashmap.len();
    }
}
