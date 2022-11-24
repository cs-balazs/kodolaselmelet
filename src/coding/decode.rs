use std::cmp::min;
use std::collections::HashSet;

fn gaussian_eliminate(mat: &mut Vec<Vec<i8>>, modulus: i8) -> Result<(), String> {
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

fn unique_permutations(items: Vec<i8>) -> Vec<Vec<i8>> {
    if items.len() == 1 {
        vec![items]
    } else {
        let mut output: Vec<Vec<i8>> = vec![];

        let mut unique_items = items.clone();
        unique_items.sort();
        unique_items.dedup();

        for first in unique_items {
            let mut remaining_elements = items.clone();

            let index = remaining_elements.iter().position(|x| *x == first).unwrap();
            remaining_elements.remove(index);

            for mut permutation in unique_permutations(remaining_elements) {
                permutation.insert(0, first.clone());
                output.push(permutation);
            }
        }
        output
    }
}

fn extract_last_n_cols(mat: &Vec<Vec<i8>>, cols_to_skip: usize) -> Vec<Vec<i8>> {
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

fn append_identity_matrix(mat: &mut Vec<Vec<i8>>) {
    let num_of_rows_to_append = mat.first().expect("Expected at least one row").len();
    for i in 0..num_of_rows_to_append {
        let mut new_vec = vec![];
        for j in 0..num_of_rows_to_append {
            new_vec.push(if i == j { 1 } else { 0 });
        }
        mat.push(new_vec);
    }
}

fn vec_mat_mul(vector: &Vec<i8>, mat: &Vec<Vec<i8>>, modulus: i8) -> Vec<i8> {
    let mut result = vec![];

    let out_len = mat.first().expect("Expected at least one row").len();

    for i in 0..out_len {
        let mut new_cell = 0;
        for (cell_index, cell) in vector.iter().enumerate() {
            new_cell += cell * mat[cell_index][i];
        }
        result.push(new_cell);
    }

    result.iter_mut().for_each(|cell| {
        *cell %= modulus;
        *cell += modulus;
        *cell %= modulus;
    });

    result
}

// TODO: Do not generate all at once, generate vectors with n errors, and only generate the ones with n+1 errors, if decoding wasn't successful with the previous ones
fn list_possible_errors(length: usize, error_count: usize, modulus: i8) -> Vec<Vec<i8>> {
    let mut errors = vec![];
    let mut new_error_vector = vec![];
    for _ in 0..length {
        new_error_vector.push(0);
    }
    errors.push(vec![new_error_vector]);

    for i in 1..=error_count {
        let mut new_err_vectors = vec![];
        for j in 1..modulus {
            for err_vector in &errors[i - 1] {
                let mut new_error_vector = err_vector.clone();
                new_error_vector[i - 1] = j;
                new_err_vectors.push(new_error_vector)
            }
        }
        errors.push(new_err_vectors);
    }

    errors.remove(0);

    let mut set = HashSet::<Vec<i8>>::new();

    errors.iter().for_each(|i| {
        i.iter().for_each(|row| {
            for perm in unique_permutations(row.to_vec()) {
                set.insert(perm);
            }
        });
    });

    Vec::from_iter(set.iter().cloned())
}

pub fn syndrome_decoder<'a>(
    input: &'a Vec<i8>,
    generator_matrix: &'a Vec<Vec<i8>>,
    modulus: i8,
) -> Result<(Vec<i8>, Vec<i8>), (String, Option<Vec<Vec<i8>>>)> {
    let mut generator = generator_matrix.clone();

    gaussian_eliminate(&mut generator, modulus).unwrap();

    if input.is_empty() {
        return Err((String::from("Recieved invalid input vector!"), None));
    }

    if generator.is_empty() || generator.iter().any(|row| row.len() != input.len()) {
        return Err((String::from("Recieved invalid generator matrix!"), None));
    }

    // dbg!(&input);
    // dbg!(&generator);

    let mut p = extract_last_n_cols(&generator, generator.len());

    // dbg!(&p);

    p.iter_mut()
        .for_each(|row| row.iter_mut().for_each(|cell| *cell *= -1));

    append_identity_matrix(&mut p);

    p.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|cell| {
            *cell %= modulus;
            *cell += modulus;
            *cell %= modulus;
        })
    });

    // dbg!(&p);

    let syndrome = vec_mat_mul(&input, &p, modulus);

    // dbg!(&syndrome);

    if syndrome.iter().all(|item| *item == 0) {
        return Ok((input.clone()[0..generator.len()].to_vec(), vec![]));
    }

    for err_count in 1..=input.len() {
        let h = list_possible_errors(input.len(), err_count, modulus);

        // dbg!(&h);

        let mut results = vec![];

        for error_vector in h {
            let res = vec_mat_mul(&error_vector, &p, modulus);

            let is_equal_to_syndrome = res
                .iter()
                .enumerate()
                .all(|(index, err_bit)| *err_bit == syndrome[index]);

            if is_equal_to_syndrome {
                results.push(error_vector)
            }
        }

        if results.len() == 0 {
            continue;
        }

        // dbg!(&results);

        if results.len() == 1 {
            let mut corrected = input.clone();
            corrected
                .iter_mut()
                .enumerate()
                .for_each(|(index, item)| *item -= results.first().unwrap()[index]);

            corrected.iter_mut().for_each(|cell| {
                *cell %= modulus;
                *cell += modulus;
                *cell %= modulus;
            });
            return Ok((
                corrected[0..generator.len()].to_vec(),
                results.first().unwrap().to_vec(),
            ));
        }

        // dbg!(&results);

        return Err((
            String::from("Multiple closest codewords found"),
            Some(results),
        ));
    }

    Err((String::from("Failed to decode"), None))
}
