struct Solution {}

#[derive(Clone, Copy)]
enum Token {
    Num(i32),
    Op(u8),
}
impl Into<Token> for String {
    fn into(self) -> Token {
        let s = self.as_bytes();
        if s.len() > 1 {
            Token::Num(self.parse::<i32>().unwrap())
        } else {
            match s[0] {
                b'+' | b'-' | b'*' | b'/' => Token::Op(s[0]),
                _ => Token::Num(self.parse::<i32>().unwrap()),
            }
        }
    }
}

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = vec![];

        for t in tokens {
            let token: Token = t.into();
            match token {
                Token::Num(num) => {
                    stack.push(num);
                }
                Token::Op(op) => {
                    let n2 = stack.pop().unwrap();
                    let n1 = stack.pop().unwrap();
                    stack.push(match op {
                        b'+' => n1 + n2,
                        b'-' => n1 - n2,
                        b'*' => n1 * n2,
                        b'/' => n1 / n2,
                        _ => panic!(),
                    });
                }
            }
        }

        stack.pop().unwrap()
    }
}

fn main() {
    // println!("{:?}", Solution);
}
