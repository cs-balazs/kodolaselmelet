use rug::{Complete, Integer};

pub fn binary(base: &Integer, exponent: &Integer, modulus: &Integer) -> Integer {
    if *exponent == 0 {
        return Integer::from(1);
    }

    if *exponent == 1 {
        return base.clone();
    }

    let mut result = base.clone();
    result %= modulus;
    let mut exp = exponent.clone();
    let mut extra = Integer::from(1);

    while &exp > &Integer::from(1) {
        if (&exp % 2u32).complete() == Integer::from(0) {
            result *= result.clone();
            result %= modulus;
            exp /= 2;
        } else {
            extra *= &result;
            result *= result.clone();
            result %= modulus;
            exp = (exp - 1) / 2;
        }
    }

    (result * extra) % modulus
}

#[cfg(test)]
mod tests {
    use super::binary;
    use rug::Integer;

    #[test]
    fn binary_exponentiation() {
        let cases: [(u64, u64, u64, u64); 6] = [
            (212872347263547826, 0, 2323234, 1),
            (212, 9, 4, 0),
            (212, 94, 4311231, 3335023),
            (12311231, 1124, 87235765, 76312856),
            (
                212872347263547826,
                1,
                212872347263547826,
                212872347263547826,
            ),
            (1697111, 4426773, 8758767889702, 914112180011),
        ];

        for (base, exponent, modulus, expected_result) in cases {
            assert_eq!(
                binary(
                    &Integer::from(base),
                    &Integer::from(exponent),
                    &Integer::from(modulus)
                ),
                Integer::from(expected_result)
            );
        }
    }
}
