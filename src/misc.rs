use std::{
    cmp::{min, Ordering},
    collections::HashMap,
};

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

pub fn my_atoi(str: String) -> i32 {
    let s = str.trim().to_string();

    let mut is_pos: bool = true;

    let mut skips = 0;

    if s.starts_with('-') {
        is_pos = false;
        skips = 1;
    } else if s.starts_with('+') {
        is_pos = true;
        skips = 1;
    }

    // counting zeroes
    let mut zeroes = 0;

    for _ch in s
        .chars()
        .skip(skips)
        .take_while(|ch| ch.is_ascii_digit() && *ch == '0')
    {
        zeroes += 1;
    }

    let mut num: i64 = 0;

    for ch in s
        .chars()
        .skip(zeroes + skips)
        .take_while(|ch| ch.is_ascii_digit())
    {
        let digit: i64 = ch.to_digit(10).unwrap().into();
        num = num * 10 + digit;

        if is_pos && num > i32::MAX.into() {
            num = i32::MAX as i64;
            return num.try_into().unwrap();
        } else if !is_pos && num > i32::MAX.into() {
            num = i32::MIN as i64;
            return num.try_into().unwrap();
        }
    }

    let ret: i32 = num.try_into().unwrap();

    if !is_pos {
        return -ret;
    }

    ret
}

pub fn longest_palindrome(s: String) -> String {
    let len = s.len();

    for gap in (1..len + 1).rev() {
        let mut start: usize = 0;
        let mut end: usize = start + gap;

        while end <= len {
            let sub = &s[start..end];

            if is_palindrome(sub.to_string()) {
                return sub.to_string();
            }

            start += 1;
            end += 1;
        }
    }

    String::from("")
}

fn is_palindrome(str: String) -> bool {
    for itr in str.chars().zip(str.chars().rev()) {
        let (lch, rch) = itr;

        if lch != rch {
            return false;
        }
    }

    true
}

pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut count: i32 = 0;
    let mut hm: HashMap<i32, i32> = HashMap::new();

    let mut pre_sum: i32 = 0;

    hm.insert(0, 1);

    for &num in nums.iter() {
        pre_sum += num;
        let key = pre_sum - k;

        if hm.contains_key(&key) {
            count += *hm.get(&key).unwrap();
        }

        *hm.entry(pre_sum).or_insert(0) += 1;
    }

    count
}

pub fn max_product(nums: Vec<i32>) -> i32 {
    let mut res = nums[0];

    let mut maxi: i32 = nums[0];
    let mut mini: i32 = nums[0];

    for &num in nums.iter().skip(1) {
        let temp = maxi;
        maxi = std::cmp::max(num, std::cmp::max(maxi * num, mini * num));
        mini = std::cmp::min(num, std::cmp::min(num * temp, num * mini));

        res = std::cmp::max(res, std::cmp::max(maxi, mini));
    }

    res
}

pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let len = arr.len();
    let mut r: i32 = arr[0];
    let mut s: Vec<(usize, i32)> = vec![(0, arr[0])];
    let mut dp: Vec<i32> = vec![0; len];
    dp[0] = r;
    for i in 1..len {
        let mut t: i32 = arr[i];
        let mut n: usize = s.len();

        while n > 0 {
            if s[n - 1].1 > arr[i] {
                s.pop();
                n -= 1;
            } else {
                break;
            }
        }

        if n == 0 {
            t = (t + (i as i32 * arr[i])) % MOD;
        } else {
            let j = s[n - 1].0;
            t = (t + ((i - j - 1) as i32) * arr[i]) % MOD;
            t = (t + dp[j]) % MOD;
        }

        r = (r + t) % MOD;
        dp[i] = t;

        s.push((i, arr[i]));
    }
    r
}

pub fn generate_pascal(num_rows: i32) -> Vec<Vec<i32>> {
    let mut ret: Vec<Vec<i32>> = vec![];

    ret.push(vec![1]);
    if num_rows == 1 {
        return ret;
    }

    ret.push(vec![1, 1]);
    if num_rows == 2 {
        return ret;
    }

    for i in 1..num_rows - 1 {
        let prev_row: usize = i.try_into().unwrap();
        let prev = &ret[prev_row];
        let mut arr: Vec<i32> = vec![1];
        for j in 0..prev.len() - 1 {
            arr.push(prev[j] + prev[j + 1]);
        }

        arr.push(1);

        ret.push(arr);
    }

    ret
}

pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
    let mut st: Vec<i32> = vec![];

    for &aster in asteroids.iter() {
        let mut attacker: i32 = aster;
        let mut dest: bool = false;
        while !st.is_empty() && attacker < 0 && st.last().unwrap() * attacker < 0 {
            let target = st.pop().unwrap();
            match target.abs().cmp(&attacker.abs()) {
                Ordering::Equal => {
                    dest = true;
                    break;
                }
                Ordering::Greater => attacker = target,
                Ordering::Less => (),
            }
        }
        if !dest {
            st.push(attacker);
        }
    }

    st
}

pub fn remove_kdigits(num: String, m: i32) -> String {
    let mut st: Vec<char> = vec![];
    let mut k: i32 = m;
    for ch in num.chars() {
        while k > 0
            && !st.is_empty()
            && st.last().unwrap().to_digit(10).unwrap() > ch.to_digit(10).unwrap()
        {
            st.pop();
            k -= 1;
        }

        st.push(ch);
    }

    while k > 0 && !st.is_empty() {
        st.pop();
        k -= 1;
    }
    let mut ret = st
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("");

    ret = ret.replace('0', " ");
    ret = ret.trim_start().to_string();
    ret = ret.replace(' ', "0");

    ret
}

pub fn trap(height: Vec<i32>) -> i32 {
    let len = height.len();
    let mut pre_max = vec![0; len];
    let mut post_max = vec![0; len];

    let mut max = height[0];
    for (i, &num) in height.iter().skip(1).enumerate() {
        pre_max[i + 1] = max;
        max = std::cmp::max(max, num);
    }

    max = height[len - 1];
    for (i, &num) in height.iter().rev().skip(1).enumerate() {
        post_max[len - i - 2] = max;
        max = std::cmp::max(max, num);
    }
    let mut water: i32 = 0;
    for (i, &num) in height.iter().enumerate() {
        let min = std::cmp::min(pre_max[i], post_max[i]);

        if min > num {
            water += min - num;
        }
    }

    water
}

pub fn three_sum(mut nums: Vec<i32>) -> usize {
    nums.sort();

    let mut ret: Vec<Vec<i32>> = vec![];

    for i in 0..nums.len() - 2 {
        let fixed = i;
        if fixed > 0 && nums[fixed] == nums[fixed - 1] {
            continue;
        }

        let mut start: usize = i + 1;
        let mut end: usize = nums.len() - 1;
        let search = -nums[fixed];

        while start < end {
            if (nums[start] + nums[end]) == search {
                ret.push(vec![nums[fixed], nums[start], nums[end]]);

                while start < end && nums[start] == nums[start + 1] {
                    start += 1;
                }
                while start < end && nums[end] == nums[end - 1] {
                    end -= 1;
                }
                start += 1;
                end -= 1;
            } else if (nums[start] + nums[end]) >= search {
                end -= 1;
            } else {
                start += 1;
            }
        }
    }

    ret.len()
}

pub fn find_max_length(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut prefix_sum: i32 = 0;
    let mut ans: i32 = 0;
    let mut mp: HashMap<i32, i32> = HashMap::new();
    mp.insert(0, -1);
    for (i, num) in nums.into_iter().enumerate() {
        prefix_sum += if num == 1 { 1 } else { -1 };
        if let std::collections::hash_map::Entry::Vacant(e) = mp.entry(prefix_sum) {
            e.insert(i as i32);
        } else {
            ans = std::cmp::max(ans, i as i32 - *mp.get(&prefix_sum).unwrap());
        }
    }
    ans
}

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut mp: HashMap<char, usize> = HashMap::new();
    let mut max_len: usize = 0;
    let mut prev_idx: usize = 0;
    s.char_indices().for_each(|(i, c)| {
        if let Some(&stored_idx) = mp.get(&c) {
            max_len = std::cmp::max(max_len, i - prev_idx);
            // prev_idx = std::cmp::max(prev_idx,stored_idx+1);
            prev_idx = stored_idx + 1;
        }
        mp.insert(c, i);
    });

    max_len = std::cmp::max(max_len, s.len() - prev_idx);
    max_len as i32
}
