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
