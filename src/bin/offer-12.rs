struct Solution {}

impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let word = word.chars().collect::<Vec<_>>();
        let mut visited = vec![vec![false; board[0].len()]; board.len()];

        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if Self::dfs(
                    &mut board,
                    &word,
                    &mut visited,
                    i as isize,
                    j as isize,
                    0,
                ) {
                    return true;
                }
            }
        }

        false
    }

    fn dfs(
        board: &mut Vec<Vec<char>>,
        word: &Vec<char>,
        visited: &mut Vec<Vec<bool>>,
        i: isize,
        j: isize,
        k: usize,
    ) -> bool {
        if i >= board.len() as isize
            || j >= board[0].len() as isize
            || i < 0
            || j < 0
            || visited[i as usize][j as usize]
        {
            return false;
        }

        if board[i as usize][j as usize] != word[k] {
            return false;
        }

        if k == word.len() - 1 {
            return true;
        }

        visited[i as usize][j as usize] = true;
        let res = Self::dfs(board, word, visited, i + 1, j, k + 1)
            || Self::dfs(board, word, visited, i, j + 1, k + 1)
            || Self::dfs(board, word, visited, i - 1, j, k + 1)
            || Self::dfs(board, word, visited, i, j - 1, k + 1);
        visited[i as usize][j as usize] = false;

        res
    }
}

fn main() {}
