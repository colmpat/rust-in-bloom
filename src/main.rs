mod bloom_filter;

use bloom_filter::BloomFilter;

fn main() {
    let m = 10 * 1024 * 1024;
    let k = 10;

    let mut bf = BloomFilter::new(m, k);

    // add a string to the set
    println!("Adding 'hello' to the set");
    bf.put(&"hello");

    // check if a string is in the set
    println!("'hello' is in the set: {}", bf.get(&"hello"));
}
