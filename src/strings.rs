pub fn check_palindrome(s: String) -> bool {
    let mut str = s.clone();

    str.make_ascii_lowercase();

    let b = str
        .chars()
        .filter(|ch| ch.is_alphanumeric())
        .collect::<String>()
        .into_bytes();

    let len = b.len();
    let mut start: usize = 0;
    let mut end: usize = len - 1;

    while b[start] == b[end] {
        if start >= end {
            return true;
        }

        start += 1;
        end -= 1;
    }

    false
}

pub fn reverse(str: String) -> String {
    let s = str.clone();
    let arr: Vec<&str> = s.split_whitespace().rev().collect();
    arr.join(" ")
}

pub fn is_subsequence(s: String, t: String) -> bool {
    if s.is_empty() {
        return true;
    }

    let mut t_arr = t.chars();

    let s_itr = s.chars();

    for sch in s_itr {
        loop {
            match t_arr.next() {
                Some(tch) if tch == sch => break,
                None => return false,
                _ => (),
            }
        }
    }

    true
}
