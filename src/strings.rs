use std::collections::HashSet;

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

pub fn anagram(s: String, t: String) -> bool {
    let mut svec = s.chars().collect::<Vec<char>>();
    let mut tvec = t.chars().collect::<Vec<char>>();

    svec.sort_unstable();
    tvec.sort_unstable();

    svec == tvec
}

pub fn left_most_repeating_index(s: String) -> usize {
    let store = &mut [-1; 256];

    let arr = s.into_bytes();

    for (index, &bval) in arr.iter().enumerate() {
        let bvali: usize = bval.into();

        let num = &mut store[bvali];

        if *num != -1 {
            return (*num).try_into().unwrap();
        } else {
            *num = index as i32;
        }
    }

    unreachable!("Invalid input")
}

pub fn left_most_repeating_char(s: String) -> char {
    let mut hs = HashSet::new();
    s.chars().find(|&c| !hs.insert(c)).unwrap()
}

pub fn left_most_non_repeating(s: String) -> i32 {
    let arr = s.into_bytes();
    let store = &mut [-1; 256];

    for &by in arr.iter() {
        let index: usize = by.into();
        let val = &mut store[index];

        *val += 1;
    }

    for (i, &by) in arr.iter().enumerate() {
        let index: usize = by.into();
        let val = store[index];
        if val == 0 {
            return i as i32;
        }
    }

    -1
}

// ---------- pattern search ----------

pub fn naive_search(text: String, patt: String) -> Vec<usize> {
    let mut indexs = Vec::new();
    let m = text.len();
    let n = patt.len();

    for i in 0..(m - n + 1) {
        let mut matched: bool = true;
        let mut textitr = text.chars().skip(i);
        for pch in patt.chars() {
            match textitr.next() {
                Some(tch) if tch != pch => {
                    matched = false;
                    break;
                }
                None => return indexs,
                _ => (),
            }
        }

        if matched {
            indexs.push(i);
        }
    }

    indexs
}

const Q: i32 = 1000;
const D: i32 = 10;
pub fn rabin_karp(mut patt: String, mut text: String) -> usize {
    let mut matched: Vec<usize> = Vec::new();

    let m: usize = patt.len();
    let n: usize = text.len();

    patt = patt.to_lowercase();
    text = text.to_lowercase();

    let bpatt = patt.clone().into_bytes();
    let btext = text.clone().into_bytes();

    let mut h: i32 = 1;

    // calc d^m-1%q
    for _i in 0..m {
        h = (h * D) % Q;
    }

    // calc t and p
    let mut p: i32 = 0;
    let mut t: i32 = 0;

    for i in 0..m {
        p = ((p * D) + (bpatt[i] as i32)) % Q;
        t = ((t * D) + (btext[i] as i32)) % Q;
    }

    for i in 0..(n - m + 1) {
        if p == t {
            let mut flag: bool = false;
            let mut titr = text.chars().skip(i);
            for pch in patt.chars() {
                match titr.next() {
                    Some(tch) if tch != pch => {
                        flag = true;
                        break;
                    }
                    None => return matched.len(),
                    _ => (),
                }
            }

            if !flag {
                matched.push(i);
            }
        }
        // donot calculate t for the last counter since there is no comparison after it.
        if i < (n - m) {
            t = (D * (t - ((btext[i] as i32) * h)) + (btext[i + m] as i32)) % Q;

            if t < 0 {
                t += Q;
            }
        }
    }
    matched.len()
}
