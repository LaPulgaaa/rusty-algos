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
