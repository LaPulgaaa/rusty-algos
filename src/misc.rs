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
