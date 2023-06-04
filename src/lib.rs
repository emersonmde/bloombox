use serde::{Serialize, Deserialize};
use std::{hash::{Hash, Hasher}, f64::consts::LN_2};
use twox_hash::XxHash;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BloomBox {
    bit_vector: Vec<bool>,
    seeds: Vec<u64>,
    size: usize,
    pub insert_count: usize,
}

impl BloomBox {
    pub fn new(size: usize, seeds: Vec<u64>) -> BloomBox {
        BloomBox {
            bit_vector: vec![false; size],
            seeds,
            size,
            insert_count: 0,
        }
    }

    /// Creates a new `BloomBox` based on the desired false positive rate and expected number of items.
    ///
    /// The size of the bit array and the number of hash functions are automatically calculated
    /// to be optimal for the provided false positive rate and expected number of items.
    ///
    /// # Arguments
    ///
    /// * `false_positive_rate` - The desired false positive rate (e.g., 0.01 for 1%)
    /// * `expected_num_items` - The expected number of items to be inserted into the BloomBox
    ///
    /// # Returns
    ///
    /// A new `BloomBox` optimized for the provided false positive rate and expected number of items.
    pub fn with_rate(false_positive_rate: f64, expected_num_items: usize) -> BloomBox {
        let ln_p = false_positive_rate.ln();
        let n = expected_num_items as f64;
        let m = (-n * ln_p / LN_2.powi(2)).ceil() as usize;
        let k = (m as f64 / n * LN_2).ceil() as u64;

        // Generate seeds for the hash functions
        let seeds = (0..k).collect();

        BloomBox {
            bit_vector: vec![false; m],
            seeds,
            size: m,
            insert_count: 0,
        }
    }

    fn hash_item<T: Hash>(&self, item: &T, seed: u64) -> usize {
        let mut hasher = XxHash::with_seed(seed);
        item.hash(&mut hasher);
        hasher.finish() as usize % self.size
    }

    pub fn insert<T: Hash>(&mut self, item: &T) {
        for &seed in &self.seeds {
            let hashed = self.hash_item(item, seed);
            self.bit_vector[hashed] = true;
        }
        self.insert_count += 1;
    }

    pub fn contains<T: Hash>(&self, item: &T) -> bool {
        for &seed in &self.seeds {
            let hashed = self.hash_item(item, seed);
            if !self.bit_vector[hashed] {
                return false;
            }
        }
        true
    }

    pub fn get_insert_count(&self) -> usize {
        self.insert_count
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get_seeds(&self) -> &Vec<u64> {
        &self.seeds
    }

    pub fn get_num_seeds(&self) -> usize {
        self.seeds.len()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_deserialize() {
        let seeds = vec![1, 2, 3, 4, 5];
        let size = 100;

        let mut bloom_box = BloomBox::new(size, seeds.clone());
        bloom_box.insert(&"test");

        let serialized = serde_json::to_string(&bloom_box).unwrap();
        let deserialized: BloomBox = serde_json::from_str(&serialized).unwrap();

        // The deserialized BloomBox should have the same properties as the original.
        assert_eq!(deserialized.size, bloom_box.size);
        assert_eq!(deserialized.seeds, bloom_box.seeds);

        // The deserialized BloomBox should behave the same as the original.
        assert!(deserialized.contains(&"test"));
        assert!(!deserialized.contains(&"not_test"));
    }

    #[test]
    fn test_insert_contains() {
        let seeds = vec![1, 2, 3, 4, 5];
        let size = 1000;
        let mut bloom_box = BloomBox::new(size, seeds);

        let large_number_of_items: Vec<_> = (0..1000).map(|i| i.to_string()).collect();

        for item in &large_number_of_items {
            bloom_box.insert(item);
        }

        for item in &large_number_of_items {
            assert!(bloom_box.contains(item));
        }
    }

    #[test]
    fn test_false_positive_rate() {
        let seeds = vec![1, 2, 3, 4, 5];
        let size = 1000000;
        let mut bloom_box = BloomBox::new(size, seeds);

        // Insert initial set of items
        let inserted_items: Vec<_> = (0..10000).map(|i| i.to_string()).collect();
        for item in &inserted_items {
            bloom_box.insert(item);
        }

        // Check a different set of items, should be all negative
        let checked_items: Vec<_> = (10000..20000).map(|i| i.to_string()).collect();
        // Collect the false positives
        let false_positives: Vec<_> = checked_items.iter().filter(|item| bloom_box.contains(item)).collect();

        // False positive rate should be about, but not exactly, 1%
        let false_positive_rate = false_positives.len() as f64 / checked_items.len() as f64;
        assert!((false_positive_rate - 0.01).abs() < 0.5);
    }
}
