use std::collections::HashSet;

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

pub fn vec_mat_mul(vector: &Vec<i8>, mat: &Vec<Vec<i8>>, modulus: i8) -> Vec<i8> {
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
pub fn list_possible_errors(length: usize, error_count: usize, modulus: i8) -> Vec<Vec<i8>> {
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
