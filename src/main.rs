use rug::Integer;
use std::str::FromStr;

fn main() {
    println!(
        "{}",
        kodolaselmelet::primality::miller_rabin(Integer::from_str("173874373").unwrap())
    );
}
