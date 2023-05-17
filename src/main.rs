mod bloom_filter;
use std::{hash::Hash, fmt::Debug};

use bloom_filter::BloomFilter;

impl BloomFilter {
    fn put_print<T: Hash + Debug>(&mut self, val: T) {
        println!("putting {:?}", val);
        self.put(val);
    }

    fn get_print<T: Hash + Debug>(&mut self, val: T) {
        println!("getting {:?}", val);
        println!("{}", self.get(val));
    }
}

fn main() {
    let m = 10 * 1024 * 1024;
    let k = 10;

    let mut bf = BloomFilter::new(m, k);

    bf.put_print("hello");
    bf.get_print("hello");

    bf.put_print("world");
    bf.get_print("world");

    bf.get_print("foo");
    bf.get_print("bar");

    bf.put_print("foo");
    bf.put_print("bar");
    bf.put_print("baz");

    bf.get_print("bar");
}
