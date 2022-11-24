use std::cmp::min;

pub fn gaussian_eliminate(mat: &mut Vec<Vec<i8>>, modulus: i8) -> Result<(), String> {
    let rows_count = mat.len();
    let cols_count = mat
        .first()
        .expect("Expected at least one row in the matrix.")
        .len();

    for pivot_index in 0..min(rows_count, cols_count) {
        let mut found_row = false;
        for possible_row_index in pivot_index..rows_count {
            if mat[possible_row_index][pivot_index] != 0 {
                found_row = true;
                let found_row = mat[possible_row_index].clone();
                mat[possible_row_index] = mat[pivot_index].clone();
                mat[pivot_index] = found_row;
                break;
            }
        }
        if !found_row {
            return Err(String::from("Failed to gaussian-eliminate matrix"));
        }

        if mat[pivot_index][pivot_index] != 1 {
            let mut found_multiplier = false;
            for i in 2..modulus {
                if (i * mat[pivot_index][pivot_index]) % modulus == 1 {
                    found_multiplier = true;
                    mat[pivot_index]
                        .iter_mut()
                        .for_each(|item| *item = (*item * i) % modulus);
                    break;
                }
            }

            if !found_multiplier {
                return Err(format!("Failed to find appropriate multiplier.\nCurrent matrix:\n{:?}\nCurrent pivot index:\n{:?}", mat, pivot_index));
            }
        }

        for row_index in 0..rows_count {
            if row_index == pivot_index {
                continue;
            }

            let increment = modulus - mat[row_index][pivot_index];
            let pivot_row = mat[pivot_index].clone();
            mat[row_index]
                .iter_mut()
                .enumerate()
                .for_each(|(index, item)| {
                    *item = (*item + (increment * pivot_row[index])) % modulus
                })
        }
    }

    Ok(())
}

pub fn extract_last_n_cols(mat: &Vec<Vec<i8>>, cols_to_skip: usize) -> Vec<Vec<i8>> {
    let mut result = vec![];

    for (row_index, row) in mat.iter().enumerate() {
        result.push(vec![]);
        for (cell_index, cell) in row.iter().enumerate() {
            if cell_index >= cols_to_skip {
                result[row_index].push(*cell);
            }
        }
    }

    result
}

pub fn append_identity_matrix(mat: &mut Vec<Vec<i8>>) {
    let num_of_rows_to_append = mat.first().expect("Expected at least one row").len();
    for i in 0..num_of_rows_to_append {
        let mut new_vec = vec![];
        for j in 0..num_of_rows_to_append {
            new_vec.push(if i == j { 1 } else { 0 });
        }
        mat.push(new_vec);
    }
}

lazy_static! {
    pub static ref GOLAY_GENERATOR_MATRIX: Vec<Vec<i8>> = vec![
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,],
        vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 1, 0, 0, 0, 1, 0,],
        vec![0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 1, 0, 0, 0, 1, 0, 1,],
        vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1,],
        vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 0,],
        vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 0, 1,],
        vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1,],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1,],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 0,],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 0, 0,],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 0, 0,],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 0, 0, 1,],
    ];
}
