struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() <= 1 {
            return 0;
        }

        let mut history_max = 0;
        let mut min_before = prices[0];

        for p in prices.iter().skip(1) {
            let profit = p - min_before;
            if profit > history_max {
                history_max = profit;
            }
            if *p < min_before {
                min_before = *p;
            }
        }

        history_max
    }
}

fn main() {}
