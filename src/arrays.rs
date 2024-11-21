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
