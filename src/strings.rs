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
