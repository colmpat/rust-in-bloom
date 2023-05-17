use std::hash::{Hash, Hasher};
use seahash::SeaHasher;
use rand::Rng;

pub struct BloomFilter {
    bits: Vec<bool>,
    hash_functions: Vec<SeaHasher>,
}

impl BloomFilter {
    pub fn new(m: usize, k: usize) -> BloomFilter {
        let mut rng = rand::thread_rng();
        BloomFilter {
            bits: vec![false; m],
            hash_functions: (0..k)
                .map(|_| {
                    let seed:Vec<u64> = (0..4).map(|_| rng.gen()).collect();
                    SeaHasher::with_seeds(seed[0], seed[1], seed[2], seed[3])
                })
                .collect(),
        }
    }

    // adds the string to the set
    pub fn put<T: Hash>(&mut self, val: &T) {
        for hash in self.hash(val) {
            self.bits[hash] = true;
        }
    }

    // returns false if s is definitely not in the set and true if s might be in the set
    pub fn get<T: Hash>(&self, val: &T) -> bool {
        for hash in self.hash(val) {
            if !self.bits[hash] {
                return false;
            }
        }
        true
    }

    // returns a vector of indexes into the bit vector for the given string
    fn hash<T: Hash>(&self, val: T) -> Vec<usize> {
        let mut hashes = Vec::new();
        for hash_function in &self.hash_functions {
            let mut hasher = hash_function.clone();
            val.hash(&mut hasher);
            let hash = hasher.finish() as usize;
            hashes.push(hash % self.bits.len());
        }
        hashes
    }
}
