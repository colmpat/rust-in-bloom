pub struct BloomFilter {
    bits: BitVec,
    k: usize,
}

struct BitVec {
    bits: Vec<u8>,
}

impl BloomFilter {
    pub fn new(k: usize, count: usize) -> BloomFilter {
        BloomFilter {
            bits: BitVec::new(),
            k,
        }
    }
}

impl BitVec {
    pub fn new() -> BitVec {
        BitVec { bits: vec![] }
    }

    pub fn set(&mut self, i: usize, bit: bool) {
        let byte_index = i >> 3;
        let bit_index = i & 0b111;

        // resize the vector if necessary
        if self.bits.len() <= byte_index {
            self.bits.resize(byte_index + 1, 0);
        }

        // set the bit
        if bit {
            self.bits[byte_index] |= 1 << bit_index;
        } else {
            self.bits[byte_index] &= !(1 << bit_index);
        }
    }

    pub fn get(&self, i: usize) -> bool {
        let byte_index = i >> 3;
        let bit_index = i & 0b111;

        // return false if the vector is too small
        if self.bits.len() <= byte_index {
            return false;
        }

        // return the bit
        let bit_identifier = 1 << bit_index;
        return (self.bits[byte_index] & bit_identifier) != 0
    }
}
