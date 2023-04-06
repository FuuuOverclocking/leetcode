struct Solution {}

impl Solution {
    pub fn minimum_time(n: i32, mut relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        let n = n as usize;
        // 邻接表，但是前向
        let adj = {
            let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
            for r in relations.iter_mut() {
                r[0] -= 1;
                r[1] -= 1;
                adj[r[1] as usize].push(r[0] as usize);
            }
            adj
        };
        // 由于 time[i] >= 0，所以 0 可以标记该值无效
        let mut mem = vec![0; n];

        fn eval(i: usize, mem: &mut Vec<i32>, adj: &Vec<Vec<usize>>, time: &Vec<i32>) -> i32 {
            if mem[i] != 0 {
                return mem[i];
            }
            let mut max_time = 0;
            for &course in adj[i].iter() {
                max_time = max_time.max(eval(course, mem, adj, time));
            }
            let ret = max_time + time[i];
            mem[i] = ret;
            ret
        }

        (0..n).map(|i| eval(i, &mut mem, &adj, &time)).max().unwrap()
    }
}

fn main() {}
