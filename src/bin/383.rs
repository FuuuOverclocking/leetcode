struct Solution {}

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let ransom_note = ransom_note.as_bytes();
        let magazine = magazine.as_bytes();

        let mut dict = vec![0; 128];

        for &ch in magazine {
            dict[ch as usize] += 1;
        }

        for &ch in ransom_note {
            dict[ch as usize] -= 1;
            if dict[ch as usize] < 0 {
                return false;
            }
        }

        true
    }
}

fn main() {}
