use std::cmp::Ordering;

use crate::partition::lomutopart;

pub fn quick_select(arr: &mut [i32], k: usize) -> i32 {
    let (mut l, mut h): (usize, usize) = (0, arr.len() - 1);

    while l <= h {
        let part = lomutopart(arr, l, h);

        match part.cmp(&(k - 1)) {
            Ordering::Equal => {
                return arr[part];
            }
            Ordering::Less => {
                l = part - 1;
            }
            Ordering::Greater => {
                h = part - 1;
            }
        }
    }

    -1
}

pub fn get_majority(arr: &[i32]) -> i32 {
    let mut leader: i32 = arr[0];
    let mut count: i32 = 1;

    for num in arr.iter().skip(1) {
        if *num == leader {
            count += 1;
        } else {
            count -= 1;

            if count < 0 {
                leader = *num;
                count = 1;
            }
        }
    }
    count = 0;
    for x in arr {
        if *x == leader {
            count += 1;
        }
    }

    if count > (arr.len() / 2).try_into().unwrap() {
        return leader;
    }

    -1
}
