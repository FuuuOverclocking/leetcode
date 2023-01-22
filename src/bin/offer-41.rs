struct MedianFinder {
    arr: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    /** initialize your data structure here. */
    fn new() -> Self {
        MedianFinder { arr: Vec::new() }
    }

    fn add_num(&mut self, num: i32) {
        if self.arr.is_empty() {
            self.arr.push(num);
            return;
        }

        let mut i = self.arr.len();
        self.arr.push(num);

        loop {
            if i == 0 {
                break;
            }
            if self.arr[i - 1] < self.arr[i] {
                break;
            }
            self.arr.swap(i - 1, i);
            i -= 1;
        }
    }

    fn find_median(&self) -> f64 {
        if self.arr.len() % 2 == 1 {
            return self.arr[self.arr.len() / 2] as f64;
        }
        let a = self.arr[self.arr.len() / 2];
        let b = self.arr[self.arr.len() / 2 - 1];
        return ((a + b) as f64) / 2.;
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */

fn main() {}
