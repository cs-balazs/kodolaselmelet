use crate::{errors::NotPrimeError, exponentiation::binary, primality::miller_rabin};
use rug::{rand::RandState, Complete, Integer};
use std::ops::Add;

// TODO: Reimplement this
fn extended_gdc(a: &Integer, b: &Integer) -> (Integer, Integer) {
    let (mut old_r, mut r) = (a.clone(), b.clone());
    let (mut old_s, mut s) = (Integer::from(1), Integer::from(0));
    let (mut old_t, mut t) = (Integer::from(0), Integer::from(1));

    while r.clone() != Integer::from(0) {
        let qoutient = (&old_r / &r).complete();
        let old_rr = old_r.clone();
        old_r = r.clone();
        r = old_rr - &qoutient * r;

        let old_ss = old_s.clone();
        old_s = s.clone();
        s = old_ss - &qoutient * s;

        let old_tt = old_t.clone();
        old_t = t.clone();
        t = old_tt - &qoutient * t;
    }

    (old_s, old_t)
}

#[derive(Debug)]
pub struct PublicKey {
    pub n: Integer,
    pub e: Integer,
}

#[derive(Debug)]
pub struct KeyPair {
    pub pubkey: PublicKey,
    pub privkey: Integer,
}

pub fn get_keys(p: &Integer, q: &Integer) -> Result<KeyPair, NotPrimeError> {
    if !miller_rabin(p) || !miller_rabin(q) {
        return Err(NotPrimeError);
    }
    let n = (p * q).complete();
    let lambda_n = (p - Integer::from(1)).lcm(&(q - Integer::from(1)));
    let mut rng = RandState::new();
    let mut e = (&lambda_n - Integer::from(1))
        .random_below(&mut rng)
        .add(Integer::from(1));
    while e.clone().gcd(&lambda_n) != Integer::from(1) {
        e = (&lambda_n - Integer::from(1))
            .random_below(&mut rng)
            .add(Integer::from(1))
    }

    let (x, y) = extended_gdc(&lambda_n, &e);

    let x_check = (&x * &e).complete() % &lambda_n;

    let mut d = if x_check == Integer::from(1) || x_check == Integer::from(1) - &lambda_n {
        x
    } else {
        y
    };

    while d < 0 {
        d += &lambda_n;
    }

    let pubkey = PublicKey { n, e };
    Ok(KeyPair { pubkey, privkey: d })
}

pub fn encrypt(m: &Integer, pubkey: &PublicKey) -> Integer {
    binary(m, &pubkey.e, Some(&pubkey.n))
}

pub fn decrypt(c: &Integer, keys: &KeyPair) -> Integer {
    binary(c, &keys.privkey, Some(&keys.pubkey.n))
}

#[cfg(test)]
mod tests {
    use super::{decrypt, encrypt, get_keys};
    use rug::Integer;
    use std::str::FromStr;

    #[test]
    fn test() {
        let keys = get_keys(
            &Integer::from_str("4").unwrap(),
            &Integer::from_str("7").unwrap(),
        );

        if let Ok(_) = keys {
            assert!(false);
        }

        let messages = [
            Integer::from_str("2635762534782543872534872543872534825487").unwrap(),
            Integer::from_str("7346836458").unwrap(),
            Integer::from_str("1034719849287359278398124").unwrap(),
            Integer::from_str("118640265273562873648273465287345623").unwrap(),
            Integer::from_str("93864293628736528534236023782698726538762323").unwrap(),
            Integer::from_str("745638745692872083759384753876502802248237645827364511").unwrap(),
        ];

        let keys = get_keys(
            &Integer::from_str("691826793068458536074208355133049291478531419048941848702313338608164245322895819651978896147719733508565736653903956362824739385732131890969671002559232884926169408928219198718725581288282235796285353558468100964394244945998514012276023216327756247926340827782709941975233386336751116416100943416897").unwrap(),
            &Integer::from_str("720714523785889173191354177371045760242297569261140563779852769239494236607878516643196470647354242607091192426841274197616849531050433864775643197014575443597601363224005309228633349876258651710793439200905812200126442288004063350129579058857449170104638055254079167970988227634912890827343610996771").unwrap(),
        ).unwrap();

        for message in messages {
            let encrypted = encrypt(&message, &keys.pubkey);
            assert_ne!(message, encrypted);
            let decrypted = decrypt(&encrypted, &keys);
            assert_eq!(message, decrypted)
        }
    }
}
