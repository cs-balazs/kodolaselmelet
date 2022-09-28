use rug::Integer;
use std::ops::MulAssign;

pub fn binary(base: &Integer, exponent: &Integer, modulus: Option<&Integer>) -> Integer {
    if *exponent == 0 {
        return Integer::from(1);
    }

    if *exponent == 1 {
        return base.clone();
    }

    let mut result = base.clone();
    if let Some(modul) = modulus {
        result %= modul;
    }
    let mut exp = Integer::from(2);

    while exp <= *exponent {
        result.mul_assign(result.clone());
        if let Some(modul) = modulus {
            result %= modul;
        }
        exp <<= 1;
    }

    exp >>= 1;

    if exp < *exponent {
        let expo = exponent - exp;
        let rest = binary(base, &expo, modulus);
        result = &result * rest;

        if let Some(modul) = modulus {
            result %= modul;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::binary;
    use rug::Integer;

    #[test]
    fn test() {
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
                    Some(&Integer::from(modulus))
                ),
                Integer::from(expected_result)
            );
        }
    }
}
