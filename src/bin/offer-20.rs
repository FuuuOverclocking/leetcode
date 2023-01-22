struct Solution {}

type Result<T> = std::result::Result<T, ()>;

impl Solution {
    pub fn is_number(s: String) -> bool {
        _is_number(s.as_bytes()).is_ok()
    }
}

fn at(s: &[u8], pos: usize) -> Result<u8> {
    if pos < s.len() {
        Ok(s[pos])
    } else {
        Err(())
    }
}

fn _is_number(mut s: &[u8]) -> Result<()> {
    s = parse_spaces(s)?;

    if let Ok(tmp) = parse_float(s) {
        s = tmp;
    } else if let Ok(tmp) = parse_int(s) {
        s = tmp;
    } else {
        return Err(());
    }

    if let Ok(tmp) = parse_exp(s) {
        s = tmp;
    }

    s = parse_spaces(s)?;

    if s.len() == 0 {
        return Ok(());
    } else {
        return Err(());
    }
}

fn parse_spaces(s: &[u8]) -> Result<&[u8]> {
    for (i, ch) in s.iter().enumerate() {
        if *ch == b' ' {
            continue;
        }
        return Ok(&s[i..]);
    }
    return Ok(&s[s.len()..]);
}

fn parse_float(mut s: &[u8]) -> Result<&[u8]> {
    let mut ch = at(s, 0)?;
    if ch == b'+' || ch == b'-' {
        s = &s[1..];
        ch = at(s, 0)?;
    }

    if ch == b'.' {
        s = parse_digit_fragment(&s[1..])?;
        return Ok(s);
    }

    s = parse_digit_fragment(s)?;
    ch = at(s, 0)?;

    if ch == b'.' {
        s = &s[1..];
        if let Ok(tmp) = parse_digit_fragment(s) {
            s = tmp;
        }
        return Ok(s);
    }

    return Err(());
}

fn parse_digit_fragment(s: &[u8]) -> Result<&[u8]> {
    fn is_digit(ch: u8) -> bool {
        ch >= b'0' && ch <= b'9'
    }

    if s.len() == 0 || !is_digit(s[0]) {
        return Err(());
    }

    for pos in 1..s.len() {
        if is_digit(s[pos]) {
            continue;
        }
        return Ok(&s[pos..]);
    }

    Ok(&s[s.len()..])
}

fn parse_int(mut s: &[u8]) -> Result<&[u8]> {
    let ch = at(s, 0)?;
    if ch == b'+' || ch == b'-' {
        s = &s[1..];
    }
    s = parse_digit_fragment(s)?;
    return Ok(s);
}

fn parse_exp(mut s: &[u8]) -> Result<&[u8]> {
    let ch = at(s, 0)?;
    if ch != b'e' && ch != b'E' {
        return Err(());
    }
    s = parse_int(&s[1..])?;
    return Ok(s);
}

fn main() {
    let ss = vec![
        "3.".to_string()
    ];
    
    for s in ss {
        println!("{}: {}", s, Solution::is_number(s.clone()));
    }
}
