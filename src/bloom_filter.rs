use std::{hash::{Hash, Hasher}, marker::PhantomData};
use seahash::SeaHasher;
use rand::Rng;

pub struct BloomFilter<T> {
    bits: Vec<bool>,
    hash_functions: Vec<SeaHasher>,
    pd: PhantomData<T>
}

impl<T: Hash> BloomFilter<T> {
    pub fn new(m: usize, k: usize) -> BloomFilter<T> {
        let mut rng = rand::thread_rng();
        BloomFilter {
            bits: vec![false; m],
            hash_functions: (0..k)
                .map(|_| {
                    let seed:Vec<u64> = (0..4).map(|_| rng.gen()).collect();
                    SeaHasher::with_seeds(seed[0], seed[1], seed[2], seed[3])
                })
                .collect(),
            pd: PhantomData
        }
    }

    // add an elem of type T to the set
    pub fn put(&mut self, val: &T) {
        for hash in self.hash(val) {
            self.bits[hash] = true;
        }
    }

    // check if an elem of type T is in the set
    pub fn get(&self, val: &T) -> bool {
        for hash in self.hash(val) {
            if !self.bits[hash] {
                return false;
            }
        }
        true
    }

    // hash an elem of type T to a list of indices
    fn hash(&self, val: &T) -> Vec<usize> {
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
