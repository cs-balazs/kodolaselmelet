use crate::exponentiation::{self, binary};
use rug::{rand::RandState, Integer};
use std::ops::{AddAssign, MulAssign};

fn n_factors(n: &Integer) -> Integer {
    if n % Integer::from(2u64) == Integer::from(1u64) {
        return Integer::from(0u64);
    }
    let mut exp = Integer::from(1u64);
    let mut n_clone = n.clone();
    while &n_clone % Integer::from(2u64) == Integer::from(0u64) {
        n_clone /= Integer::from(2u64);
        exp += Integer::from(1u64);
    }

    return exp - Integer::from(1u64);
}

pub fn miller_rabin(n: Integer) -> bool {
    if &n % Integer::from(2u64) == Integer::from(0u64) {
        return false;
    }
    let n_minus_one = &n - Integer::from(1u64);
    let k = n_factors(&n_minus_one);
    let r = &n_minus_one / (exponentiation::binary(&Integer::from(2u64), &k));

    println!("n = {}", n);
    println!("k = {}", k);
    println!("r = {}", r);

    let mut i = Integer::from(0u64);
    while i < k {
        println!("{}", i);
        let mut rng = RandState::new();
        let random = &n - Integer::from(3);
        let mut random = random.random_below(&mut rng);
        random.add_assign(Integer::from(2));
        println!("random = {}", random);
        println!("r = {}", r);
        let mut x = binary(&random, &r) % &n;
        println!("x = {}", x);

        if x == Integer::from(1u64) || x == n_minus_one {
            i += Integer::from(1u64);
            continue;
        }

        let mut j = Integer::from(1u64);

        while j <= &k - Integer::from(1u64) {
            x.mul_assign(x.clone());
            x.mod_u(n.to_u32_wrapping());
            if x == n_minus_one {
                i += Integer::from(1u64);
                continue;
            }
            j += Integer::from(1u64);
        }

        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::primality::miller_rabin;
    use rug::Integer;

    #[test]
    fn test() {
        assert!(miller_rabin(Integer::from(7u64)));
        assert!(!miller_rabin(Integer::from(561u64)));
        assert!(!miller_rabin(Integer::from(56112312u64)));
        assert!(!miller_rabin(Integer::from(56384756283764451u64)));
    }
}
