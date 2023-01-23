struct Solution {}

impl Solution {
    pub fn daily_temperatures(mut temps: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![];

        for i in 0..temps.len() {
            let temp = temps[i];

            if stack.is_empty() {
                stack.push((i, temp));
                continue;
            }

            while let Some((_i, _temp)) = stack.last() {
                if *_temp < temp {
                    temps[*_i] = (i - *_i) as i32;
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push((i, temp));
        }
        for (i, _) in stack {
            temps[i] = 0;
        }

        temps
    }
}

fn main() {
    // println!("{:?}", Solution);
}
