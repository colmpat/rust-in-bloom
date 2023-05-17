# rust-in-bloom

This is a Rust implementation of a Bloom filter, a probabilistic data structure used for efficient membership testing. It uses the `seahash` hash function and the `rand` crate for generating random seeds.

## Usage

1. Create a new Bloom filter by calling the `new` function and specifying the desired number of bits (`m`) and the number of hash functions (`k`):

   ```rust
   let bloom_filter = BloomFilter::new(m, k);
   ```

1. Add elements to the Bloom filter using the `put` method:

   ```rust
   bloom_filter.put(&element);
   ```

1. Check for membership of an element using the `get` method:

   ```rust
   let is_member = bloom_filter.get(&element);
   ```

## Example

Here's an example that demonstrates how to use the Bloom filter:

```rust
...
use bloom_filter::BloomFilter;

fn main() {
    let mut bloom_filter = BloomFilter::new(1000, 3);
    let element = "example";

    bloom_filter.put(&element);
    let is_member = bloom_filter.get(&element);
    println!("Is '{}' a member? {}", element, is_member);
}
```
