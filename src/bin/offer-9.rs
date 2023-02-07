struct Solution {}

#[derive(Default)]
struct CQueue {
    stack_in: Vec<i32>,
    stack_out: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CQueue {
    fn new() -> Self {
        Default::default()
    }

    fn append_tail(&mut self, value: i32) {
        self.stack_in.push(value);
    }

    fn delete_head(&mut self) -> i32 {
        if self.stack_out.is_empty() {
            while let Some(x) = self.stack_in.pop() {
                self.stack_out.push(x);
            }
        }
        self.stack_out.pop().unwrap_or(-1)
    }
}

/**
 * Your CQueue object will be instantiated and called as such:
 * let obj = CQueue::new();
 * obj.append_tail(value);
 * let ret_2: i32 = obj.delete_head();
 */

fn main() {}
