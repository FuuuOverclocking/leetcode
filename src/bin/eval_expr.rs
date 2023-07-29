fn main() {
    let expr = "1 + 22 * 3 - 5 / 9 + 1".as_bytes();

    let tokens = scan(expr);
    let ast = parse_expr(&tokens);
    let val = eval(&ast);

    println!("{val}");
}

#[derive(Clone, Copy)]
enum Token {
    Number(f64),
    Op(u8),
}

enum Expr {
    Literal(f64),
    BinaryExpr {
        op: u8,
        left_operand: Box<Expr>,
        right_operand: Box<Expr>,
    },
}

fn scan(expr: &[u8]) -> Vec<Token> {
    let mut ret = vec![];
    let mut pos = 0;

    loop {
        if pos == expr.len() {
            break;
        }
        match expr[pos] {
            b'0'..=b'9' => {
                let mut num: f64 = (expr[pos] - b'0') as f64;
                pos += 1;
                while pos < expr.len() && expr[pos] >= b'0' && expr[pos] <= b'9'
                {
                    num = num * 10. + (expr[pos] - b'0') as f64;
                    pos += 1;
                }
                ret.push(Token::Number(num));
            }
            b'+' | b'-' | b'*' | b'/' => {
                ret.push(Token::Op(expr[pos]));
                pos += 1;
            }
            b' ' => {
                pos += 1;
            }
            _ => panic!("非法字符"),
        }
    }

    ret
}

fn parse_expr(tokens: &[Token]) -> Expr {
    parse_binary_expr(tokens, 0)
}

fn parse_binary_expr(tokens: &[Token], precedence: u8) -> Expr {
    let mut pos = 0;

    fn parse(tokens: &[Token], pos: &mut usize, precedence: u8) -> Expr {
        let left = match tokens[*pos] {
            Token::Number(num) => Expr::Literal(num),
            _ => panic!("语法错误"),
        };
        *pos += 1;
        parse_rest(tokens, pos, precedence, left)
    }

    fn parse_rest(
        tokens: &[Token],
        pos: &mut usize,
        precedence: u8,
        mut left: Expr,
    ) -> Expr {
        while *pos != tokens.len() {
            let op = match tokens[*pos] {
                Token::Op(op) => op,
                _ => panic!("语法错误"),
            };

            let new_precedence = get_op_precedence(op);
            if new_precedence <= precedence {
                break;
            }

            *pos += 1;
            left = Expr::BinaryExpr {
                op,
                left_operand: Box::new(left),
                right_operand: Box::new(parse(tokens, pos, new_precedence)),
            };
        }

        left
    }

    fn get_op_precedence(op: u8) -> u8 {
        match op {
            b'+' => 1,
            b'-' => 1,
            b'*' => 2,
            b'/' => 2,
            _ => unreachable!(),
        }
    }

    parse(tokens, &mut pos, precedence)
}

fn eval(ast: &Expr) -> f64 {
    match ast {
        Expr::Literal(num) => *num,
        Expr::BinaryExpr {
            op,
            left_operand,
            right_operand,
        } => {
            let l = eval(left_operand);
            let r = eval(right_operand);

            match op {
                b'+' => l + r,
                b'-' => l - r,
                b'*' => l * r,
                b'/' => l / r,
                _ => unreachable!(),
            }
        }
    }
}
