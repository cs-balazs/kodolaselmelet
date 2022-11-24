mod mat_utils;
mod vec_utils;
pub use mat_utils::GOLAY_GENERATOR_MATRIX;

use self::{
    mat_utils::{append_identity_matrix, extract_last_n_cols, gaussian_eliminate},
    vec_utils::{list_possible_errors, vec_mat_mul},
};

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

    let mut p = extract_last_n_cols(&generator, generator.len());

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

    let syndrome = vec_mat_mul(&input, &p, modulus);

    if syndrome.iter().all(|item| *item == 0) {
        return Ok((input.clone()[0..generator.len()].to_vec(), vec![]));
    }

    for err_count in 1..=input.len() {
        let h = list_possible_errors(input.len(), err_count, modulus);

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

        return Err((
            String::from("Multiple closest codewords found"),
            Some(results),
        ));
    }

    Err((String::from("Failed to decode"), None))
}
