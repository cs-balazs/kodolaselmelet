use kodolaselmelet::coding::{self, decode::GOLAY_GENERATOR_MATRIX};

fn main() {
    let generator_matrix = vec![
        vec![1, 1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 1, 1],
        vec![1, 1, 1, 1, 0, 1],
        vec![1, 0, 1, 1, 1, 0],
    ];

    let input_vec = vec![2, 0, 2, 0, 2, 1];

    let res = coding::decode::syndrome_decoder(&input_vec, &generator_matrix, 2);

    match res {
        Err((error_msg, results)) => {
            println!("[DECODING ERROR]");
            println!("[ERROR MESSAGE] - {}", error_msg);
            if let Some(multiple_results) = results {
                println!("[POSSIBLE ERRORS] - {:?}", multiple_results);
            }
        }
        Ok((decoded, err_mask)) => {
            println!("[DECODING SUCCESSFUL]");
            println!("[DECODED MESSAGE] - {:?}", decoded);
            if err_mask.len() == 0 {
                println!("[NO ERROR WAS PRESENT]")
            } else {
                println!("[ERROR BITS] - {:?}", err_mask);
            }
        }
    }
}
