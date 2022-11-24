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

lazy_static! {
    static ref GENERATOR_MATRIX_1: Vec<Vec<i8>> = vec![
        vec![1, 1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 1, 1],
        vec![1, 1, 1, 1, 0, 1],
        vec![1, 0, 1, 1, 1, 0],
    ];
    static ref GENERATOR_MATRIX_2: Vec<Vec<i8>> = vec![vec![1, 0, 1, 2], vec![0, 1, 1, 1]];
    static ref GENERATOR_MATRIX_3: Vec<Vec<i8>> = vec![
        vec![1, 0, 0, 1, 0],
        vec![0, 1, 0, 1, 1],
        vec![0, 0, 1, 1, 0],
    ];
}

#[cfg(test)]
mod tests {
    use crate::coding::decode::{
        GENERATOR_MATRIX_1, GENERATOR_MATRIX_2, GENERATOR_MATRIX_3, GOLAY_GENERATOR_MATRIX,
    };

    #[test]
    fn invalid_input_dimensions() {
        let result = super::syndrome_decoder(&vec![2, 0, 2, 0, 2, 1, 0], &GENERATOR_MATRIX_1, 2);
        assert!(result.is_err());
        let (error_msg, results) = result.unwrap_err();
        assert!(results.is_none());
        assert_eq!(error_msg, "Recieved invalid generator matrix!");
    }

    #[test]
    fn non_gauss_eliminated_matrix_success_2_finite_field_one_error_bit() {
        let result = super::syndrome_decoder(&vec![2, 0, 2, 0, 2, 1], &GENERATOR_MATRIX_1, 2);
        assert!(result.is_ok());
        let (decoded_msg, errors) = result.unwrap();
        assert_eq!(decoded_msg, vec![0, 0, 0, 0]);
        assert_eq!(errors, vec![0, 0, 0, 0, 0, 1]);
    }

    #[test]
    fn gauss_eliminated_matrix_success_3_finite_field_no_error_bits() {
        let result = super::syndrome_decoder(&vec![2, 1, 0, 2], &GENERATOR_MATRIX_2, 3);
        assert!(result.is_ok());
        let (decoded_msg, errors) = result.unwrap();
        assert_eq!(decoded_msg, vec![2, 1]);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn gauss_eliminated_matrix_error_2_finite_field_3_closest_codewords() {
        let result = super::syndrome_decoder(&vec![0, 1, 0, 0, 1], &GENERATOR_MATRIX_3, 2);
        assert!(result.is_err());
        let (error_msg, results) = result.unwrap_err();
        assert!(results.is_some());
        let results = results.unwrap();
        assert_eq!(error_msg, "Multiple closest codewords found");

        let expected_closest_codewords = vec![
            vec![1, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 0],
            vec![0, 0, 1, 0, 0],
        ];

        assert_eq!(results.len(), expected_closest_codewords.len());
        assert!(results
            .iter()
            .all(|item| expected_closest_codewords.contains(item)));
    }

    #[test]
    fn gauss_eliminated_matrix_success_2_finite_field_one_error_bits() {
        let result = super::syndrome_decoder(&vec![1, 0, 0, 0, 1], &GENERATOR_MATRIX_3, 2);
        assert!(result.is_ok());
        let (decoded_msg, errors) = result.unwrap();
        assert_eq!(decoded_msg, vec![1, 1, 0]);
        assert_eq!(errors, vec![0, 1, 0, 0, 0]);
    }

    #[test]
    fn golay_two_error_bits() {
        let result = super::syndrome_decoder(
            &vec![
                1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 0, 0,
            ],
            &GOLAY_GENERATOR_MATRIX,
            2,
        );
        assert!(result.is_ok());
        let (decoded_msg, errors) = result.unwrap();
        assert_eq!(decoded_msg, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0]);
        assert_eq!(
            errors,
            vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn golay_three_error_bits() {
        let result = super::syndrome_decoder(
            &vec![
                1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 0, 0,
            ],
            &GOLAY_GENERATOR_MATRIX,
            2,
        );
        assert!(result.is_ok());
        let (decoded_msg, errors) = result.unwrap();
        assert_eq!(decoded_msg, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0]);
        assert_eq!(
            errors,
            vec![1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn golay_four_error_bits_should_fail() {
        let result = super::syndrome_decoder(
            &vec![
                1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 0, 0,
            ],
            &GOLAY_GENERATOR_MATRIX,
            2,
        );
        assert!(result.is_err());
        let (error_msg, results) = result.unwrap_err();
        assert!(results.is_some());
        let results = results.unwrap();
        assert_eq!(error_msg, "Multiple closest codewords found");

        let expected_closest_codewords = vec![
            vec![
                0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0,
            ],
            vec![
                0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0,
            ],
            vec![
                0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0,
            ],
            vec![
                0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1,
            ],
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0,
            ],
            vec![
                1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
        ];

        assert_eq!(results.len(), expected_closest_codewords.len());
        assert!(results
            .iter()
            .all(|item| expected_closest_codewords.contains(item)));
    }
}
