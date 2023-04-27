struct HashTable {
    size: usize,
    data: Vec<Vec<(String, i32)>>,
}

impl HashTable {
    fn new(size: usize) -> Self {
        Self {
            size,
            data: vec![Vec::new(); size],
        }
    }

    fn hash(&self, key: &str) -> usize {
        let mut hash_value: u64 = 0;
        for byte in key.bytes() {
            hash_value = hash_value.wrapping_mul(31).wrapping_add(byte as u64);
        }
        hash_value as usize % self.size
    }

    fn set(&mut self, key: String, value: i32) {
        let hash = self.hash(&key);
        let bucket = &mut self.data[hash];
        for (i, item) in bucket.iter_mut().enumerate() {
            if item.0 == key {
                item.1 = value;
                return;
            }
        }
        bucket.push((key, value));
    }

    fn get(&self, key: &str) -> Option<i32> {
        let hash = self.hash(key);
        let bucket = &self.data[hash];
        for item in bucket {
            if item.0 == key {
                return Some(item.1);
            }
        }
        None
    }

    fn delete(&mut self, key: &str) {
        let hash = self.hash(key);
        let bucket = &mut self.data[hash];
        for i in 0..bucket.len() {
            if bucket[i].0 == key {
                bucket.remove(i);
                return;
            }
        }
    }
}
