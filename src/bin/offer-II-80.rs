struct Solution {}

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut ret = vec![];

        fn work(
            ret: &mut Vec<Vec<i32>>,
            working: &mut Vec<i32>,
            l: i32,
            n: i32,
            need: usize,
        ) {
            if need == 1 {
                for num in l..n {
                    working.push(num + 1);
                    ret.push(working.clone());
                    working.pop();
                }
                return;
            }

            for i in 0..=(n - l - need as i32) {
                working.push(l + i + 1);
                work(ret, working, l + i + 1, n, need - 1);
                working.pop();
            }
        }
        let mut working = vec![];
        work(&mut ret, &mut working, 0, n, k);

        ret
    }
}

fn main() {
    Solution::combine(3, 3);
}
