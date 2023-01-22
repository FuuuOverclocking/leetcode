struct Solution {}

struct Pos {
    i: i32,
    j: i32,
}

struct DirGenerator(i32);
impl DirGenerator {
    fn get(&mut self) -> (i32, i32) {
        match self.0 {
            
        }
    }
}

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::<i32>::new();

        let mut il = 0;
        let mut ih = matrix.len() - 1;
        let mut jl = 0;
        let mut jh = matrix[0].len() - 1;
    }
}

fn main() {}
