struct Solution {}

impl Solution {
    pub fn hamming_weight(mut n: u32) -> u32 {
        n = (n & 0x55555555) + ((n & 0xAAAAAAAA) >> 1);
        n = (n & 0x33333333) + ((n & 0xCCCCCCCC) >> 2);
        n = (n & 0x0F0F0F0F) + ((n & 0xF0F0F0F0) >> 4);
        n = (n & 0x00FF00FF) + ((n & 0xFF00FF00) >> 8);
        n = (n & 0x0000FFFF) + ((n & 0xFFFF0000) >> 16);
        n
    }
}

fn main() {}
