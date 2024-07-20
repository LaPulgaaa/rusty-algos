use crate::partition::{hoare, lomutopart};

// O(N^2) O(1) stable
pub fn bubble_sort(mut arr: Vec<i32>) -> Vec<i32> {
    for i in 0..arr.len() - 1 {
        for j in 0..arr.len() - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
    arr
}

// O(N^2) O(1) stable
pub fn bubble_sort_optimal(mut arr: Vec<i32>) -> Vec<i32> {
    let mut swapped = false;

    for i in 0..arr.len() - 1 {
        for j in 0..arr.len() - i - 1 {
            if arr[j] > arr[j + 1] {
                swapped = true;
                arr.swap(j, j + 1);
            }
        }
        if !swapped {
            return arr;
        }
    }

    arr
}

// O(N^2) O(len) unstable
pub fn select_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let len: usize = arr.len();

    if len == 1 || len == 0 {
        return arr;
    }

    let mut temp = Vec::new();

    let mut i = 0;

    while i < len {
        let mut min_index = 0;
        for j in 1..arr.len() {
            if arr[min_index] > arr[j] {
                min_index = j;
            }
        }

        temp.push(arr[min_index]);
        arr[min_index] = i32::MAX;
        i += 1;
    }

    temp
}

// O(N^2) O(1) unstable
pub fn select_sort_opt(mut arr: Vec<i32>) -> Vec<i32> {
    if arr.is_empty() || arr.len() == 1 {
        return arr;
    }

    for i in 0..arr.len() {
        let mut rest_min = i;

        for j in i + 1..arr.len() {
            if arr[j] < arr[rest_min] {
                rest_min = j;
            }
        }

        arr.swap(rest_min, i);
    }

    arr
}

// O(N^2) worst case O(N) best case -- O(1)
pub fn insertion_sort(mut arr: Vec<i32>) -> Vec<i32> {
    if arr.is_empty() || arr.len() == 1 {
        return arr;
    }

    for i in 1..arr.len() {
        let key = arr[i];
        let mut j = i;

        while arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;

            if j == 0 {
                break;
            }
        }

        arr[j] = key;
    }

    arr
}

// O(NlogN) average/best O(N^2) worst case
pub fn quick_sort_lomuto(arr: &mut Vec<i32>, l: usize, h: usize) {
    if l < h {
        let part = lomutopart(arr, l, h);

        //when the partition is at index 0; there are no element before it to sort.
        if part != 0 {
            quick_sort_lomuto(arr, l, part - 1);
        }

        quick_sort_lomuto(arr, part + 1, h);
    }
}

pub fn quick_sort_hoare(arr: &mut Vec<i32>, l: usize, h: usize) {
    if l < h {
        let part = hoare(arr, l, h);
        quick_sort_hoare(arr, l, part);
        quick_sort_hoare(arr, part + 1, h);
    }
}

// O(NlogN) O(N)
fn merge(mut arr: Vec<i32>, start: usize, mid: usize, end: usize) -> Vec<i32> {
    let len1 = mid - start + 1;
    let len2 = end - mid;

    let mut arr1 = vec![0; len1];
    let mut arr2 = vec![0; len2];

    arr1.copy_from_slice(&arr[start..mid + 1]);
    arr2.copy_from_slice(&arr[mid + 1..end + 1]);

    let (mut i, mut j, mut k): (usize, usize, usize) = (0, 0, start);

    while i < len1 || j < len2 {
        if j >= len2 || (i < len1 && arr1[i] <= arr2[j]) {
            arr[k] = arr1[i];
            k += 1;
            i += 1;
        } else {
            arr[k] = arr2[j];
            k += 1;
            j += 1;
        }
    }

    arr
}

pub fn merge_sort(mut arr: Vec<i32>, start: usize, end: usize) -> Vec<i32> {
    if arr.is_empty() || arr.len() == 1 {
        return arr;
    }

    if end > start {
        let mid = start + (end - start) / 2;
        arr = merge_sort(arr, start, mid);
        arr = merge_sort(arr, mid + 1, end);
        arr = merge(arr, start, mid, end);
    }

    arr
}
