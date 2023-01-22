struct Solution {}
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a: Vec<u8> = a.as_bytes().iter().rev().map(|ch| ch - b'0').collect();
        let b: Vec<u8> = b.as_bytes().iter().rev().map(|ch| ch - b'0').collect();

        let width = usize::max(a.len(), b.len()) + 1;
        let mut c = vec![0; width];

        let mut carry = 0;
        for i in 0..width {
            let da = if i >= a.len() { 0 } else { a[i] };
            let db = if i >= b.len() { 0 } else { b[i] };
            c[i] = da + db + carry;
            if c[i] & 0b10 != 0 {
                c[i] = c[i] & 1;
                carry = 1;
            } else {
                carry = 0;
            }
        }
        if *c.last().unwrap() == 0 {
            c.pop();
        }
        String::from_utf8(c.iter().rev().map(|digit| digit + b'0').collect::<Vec<_>>()).unwrap()
    }
}

fn main() {
    // Solution::
}
