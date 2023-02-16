struct Solution {}

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0]));

        let mut curr_start = intervals[0][0];
        let mut curr_end = intervals[0][1];

        let mut ret = vec![];

        for inter in intervals.iter().skip(1) {
            let start = inter[0];
            let end = inter[1];
            if start <= curr_end {
                if end > curr_end {
                    curr_end = end;
                }
            } else {
                ret.push(vec![curr_start, curr_end]);
                curr_start = start;
                curr_end = end;
            }
        }
        ret.push(vec![curr_start, curr_end]);

        ret
    }
}
fn main() {}
