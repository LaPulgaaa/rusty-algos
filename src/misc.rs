use std::cmp::min;

// minimum difference in an array

pub fn min_diff(arr: &mut [i32]) -> i32 {
    arr.sort();

    let mut min_diff = i32::MAX;

    let mut prev: i32 = arr[0];

    for x in arr.iter().skip(1) {
        min_diff = min(min_diff, *x - prev).abs();
        prev = *x;
    }

    min_diff
}

// sort by character frequency

pub fn frequency_sort(s: String) -> String {
    let store = &mut [(0, ' '); 256];

    for ch in s.chars() {
        let index = ch as usize;
        let (freq, chr) = &mut store[index];

        *freq += 1;
        *chr = ch;
    }

    store.sort_by(|a, b| (b.0).cmp(&a.0));
    let mut ret: String = String::new();

    for &e in store.iter() {
        let (freq, ch) = e;

        if freq <= 0 {
            break;
        }

        for _i in 0..freq {
            ret.push(ch);
        }
    }

    ret
}

//remove outer_most parantheses
pub fn remove_outer_parentheses(s: String) -> String {
    let mut answer: String = String::new();
    let mut depth: i32 = 0;

    for ch in s.chars() {
        if ch == '(' {
            if depth > 0 {
                answer.push(ch);
            }

            depth += 1;
        } else {
            depth -= 1;
            if depth > 0 {
                answer.push(ch);
            }
        }
    }

    answer
}

pub fn remove_outer_parentheses_one_liner(s: String) -> String {
    let mut st = vec![];
    s.chars()
        .map(|x| match x {
            '(' => {
                st.push(true);
                if st.len() >= 2 {
                    return "(";
                }

                ""
            }
            ')' => {
                st.pop();
                if !st.is_empty() {
                    return ")";
                }

                ""
            }
            _ => "",
        })
        .collect::<String>()
}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }

    let mut lcp = strs[0].as_str();

    for str in strs.iter() {
        let mut lci: usize = 0;

        for itr in str.chars().zip(lcp.chars()) {
            let (ch, lch) = itr;

            if ch == lch {
                lci += 1;
            } else {
                break;
            }
        }

        lcp = &lcp[..lci];
    }

    lcp.to_string()
}

pub fn longest_common_prefix_one_liner(strs: Vec<String>) -> String {
    strs.into_iter()
        .reduce(|a, c| {
            a.chars()
                .zip(c.chars())
                .take_while(|(ach, cch)| ach == cch)
                .map(|(ach, _)| ach)
                .collect::<String>()
        })
        .unwrap()
}

pub fn max_depth(s: String) -> i32 {
    let mut maxd: i32 = 0;
    let mut score: i32 = 0;
    const OPEN: char = '(';
    const CLOSE: char = ')';

    for ch in s.chars() {
        if ch == OPEN {
            score += 1;
        } else if ch == CLOSE {
            score -= 1;
        }

        maxd = std::cmp::max(score, maxd);
    }

    maxd
}
