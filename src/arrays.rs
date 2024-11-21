use std::collections::HashSet;

pub fn set_zeroes(matrix: &mut [Vec<i32>]) {
    let mut marked_col: HashSet<usize> = HashSet::new();

    for row in matrix.iter_mut() {
        let has_zero = row.iter().any(|x| *x == 0);

        if !has_zero {
            continue;
        }

        row.iter_mut().enumerate().for_each(|(j, val)| {
            if *val == 0 {
                marked_col.insert(j);
            }

            *val = 0;
        });
    }

    for col in marked_col {
        matrix.iter_mut().for_each(|row| row[col] = 0);
    }
}

//rotate matrix by 90 degrees

//naive method

pub fn rotate(matrix: &mut [Vec<i32>]) {
    let mut store: Vec<i32> = Vec::new();

    for row in matrix.iter() {
        for elem in row.iter() {
            store.push(*elem);
        }
    }

    for j in 0..matrix.len() {
        for i in (0..matrix.len()).rev() {
            matrix[i][j] = store.pop().unwrap();
        }
    }
}

//efficient method
//for images problems. reverse the matrix left <-> right or up <-> down
// depending on anti/clockwise rotation and then take transpose

pub fn rotate_effi(arr: &mut [Vec<i32>]) {
    let len: usize = arr.len();

    for i in 0..len / 2 {
        for j in 0..len {
            let temp = arr[i][j];
            arr[i][j] = arr[len - i - 1][j];
            arr[len - i - 1][j] = temp;
        }
    }

    for i in 0..arr.len() {
        for j in i..arr.len() {
            let temp = arr[j][i];
            arr[j][i] = arr[i][j];
            arr[i][j] = temp;
        }
    }
}
