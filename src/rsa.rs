use crate::{errors::NotPrimeError, exponentiation::binary, primality::miller_rabin};
use rug::{rand::RandState, Complete, Integer};
use std::{mem::swap, ops::Add, str::FromStr};

// https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Example
fn bezout_coefficients(a: &Integer, b: &Integer) -> (Integer, Integer) {
    let mut quotient: Integer;
    let mut prev_remainder = a.clone();
    let mut remainder = b.clone();
    let mut prev_s = Integer::from(1);
    let mut s = Integer::from(0);
    let mut prev_t = Integer::from(0);
    let mut t = Integer::from(1);

    while remainder != 0 {
        quotient = (&prev_remainder / &remainder).complete();
        prev_remainder = &prev_remainder - (&quotient * &remainder).complete();
        swap(&mut remainder, &mut prev_remainder);
        prev_s = &prev_s - (&quotient * &s).complete();
        swap(&mut s, &mut prev_s);
        prev_t = &prev_t - (&quotient * &t).complete();
        swap(&mut t, &mut prev_t);
    }

    (prev_s, prev_t)
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

pub fn generate_keys(p: &Integer, q: &Integer) -> Result<KeyPair, NotPrimeError> {
    if !miller_rabin(p) || !miller_rabin(q) {
        return Err(NotPrimeError);
    }
    let n = (p * q).complete();
    let lambda_n = (p - Integer::from(1)).lcm(&(q - Integer::from(1)));
    let mut rng = RandState::new();
    let e: Integer = {
        let mut e: Integer;
        loop {
            e = (&lambda_n - Integer::from(1))
                .random_below(&mut rng)
                .add(Integer::from(1));

            if e.clone().gcd(&lambda_n) == 1 {
                break;
            }
        }
        e
    };

    let (mut x, mut y) = bezout_coefficients(&lambda_n, &e);

    // For making sure, they are positive
    x = ((x % &lambda_n) + &lambda_n) % &lambda_n;
    y = ((y % &lambda_n) + &lambda_n) % &lambda_n;

    let d = if (&x * &e).complete() % &lambda_n == 1 {
        x
    } else {
        y
    };

    let pubkey = PublicKey { n, e };
    Ok(KeyPair { pubkey, privkey: d })
}

pub fn encrypt(m: &Integer, pubkey: &PublicKey) -> Integer {
    binary(m, &pubkey.e, &pubkey.n)
}

pub fn decrypt(c: &Integer, keys: &KeyPair) -> Integer {
    binary(c, &keys.privkey, &keys.pubkey.n)
}

fn str_to_bigint(s: &str) -> Integer {
    Integer::from_str(
        s.as_bytes()
            .iter()
            .map(|b| {
                let string = b.to_string();
                let num_of_leading_zeroes = 3 - string.len();
                format!(
                    "{}{}{}",
                    if num_of_leading_zeroes == 2 { "0" } else { "" },
                    if num_of_leading_zeroes == 1 { "0" } else { "" },
                    string
                )
            })
            .collect::<Vec<String>>()
            .join("")
            .as_str(),
    )
    .unwrap()
}
fn bigint_to_string(bigint: &Integer) -> String {
    let string = bigint.to_string();
    let num_of_zeroes_to_prepend = string.len() % 3;
    let string = format!(
        "{}{}{}",
        if num_of_zeroes_to_prepend == 2 {
            "0"
        } else {
            ""
        },
        if num_of_zeroes_to_prepend == 1 {
            "0"
        } else {
            ""
        },
        string,
    );
    let mut bytes: Vec<u8> = vec![];
    let mut index = 0;
    while index < string.len() {
        bytes.push(string[index..=index + 2].parse::<u8>().unwrap());
        index += 3;
    }
    bytes.iter().map(|b| char::from(*b)).collect::<String>()
}

pub fn encrypt_ascii(m: String, pubkey: &PublicKey) -> Integer {
    let m = &str_to_bigint(m.as_str());
    binary(m, &pubkey.e, &pubkey.n)
}

pub fn decrypt_ascii(c: &Integer, keys: &KeyPair) -> String {
    let bigint_result = binary(c, &keys.privkey, &keys.pubkey.n);
    bigint_to_string(&bigint_result)
}

pub fn sign(message: &str, keys: &KeyPair) -> Integer {
    let c = &str_to_bigint(message);
    binary(c, &keys.privkey, &keys.pubkey.n)
}

pub fn verify(signature: &Integer, pubkey: &PublicKey) -> String {
    let bigint_result = binary(signature, &pubkey.e, &pubkey.n);
    bigint_to_string(&bigint_result)
}

#[cfg(test)]
mod tests {
    use crate::rsa::{bigint_to_string, str_to_bigint};

    use super::{decrypt, encrypt, generate_keys};
    use rug::Integer;
    use std::str::FromStr;

    #[test]
    fn key_generation() {
        let keys = generate_keys(
            &Integer::from_str("4").unwrap(),
            &Integer::from_str("7").unwrap(),
        );

        if let Ok(_) = keys {
            assert!(false);
        }
    }

    #[test]
    fn encrypt_decrypt_number() {
        let messages = [
            Integer::from_str("2635762534782543872534872543872534825487").unwrap(),
            Integer::from_str("7346836458").unwrap(),
            Integer::from_str("1034719849287359278398124").unwrap(),
            Integer::from_str("118640265273562873648273465287345623").unwrap(),
            Integer::from_str("93864293628736528534236023782698726538762323").unwrap(),
            Integer::from_str("745638745692872083759384753876502802248237645827364511").unwrap(),
        ];

        let keys = generate_keys(
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

    #[test]
    fn encrypt_decrypt_ascii_str() {
        let messages = [
            "almafa",
            "egy aprocska kalapocska",
            "0123456789",
            "abscefghijklmnopqrstuvwxyz",
        ];

        let keys = generate_keys(
            &Integer::from_str("691826793068458536074208355133049291478531419048941848702313338608164245322895819651978896147719733508565736653903956362824739385732131890969671002559232884926169408928219198718725581288282235796285353558468100964394244945998514012276023216327756247926340827782709941975233386336751116416100943416897").unwrap(),
            &Integer::from_str("720714523785889173191354177371045760242297569261140563779852769239494236607878516643196470647354242607091192426841274197616849531050433864775643197014575443597601363224005309228633349876258651710793439200905812200126442288004063350129579058857449170104638055254079167970988227634912890827343610996771").unwrap(),
        ).unwrap();

        for message in messages {
            let m = str_to_bigint(message);
            let encrypted = encrypt(&m, &keys.pubkey);
            assert_ne!(m, encrypted);
            let decrypted = decrypt(&encrypted, &keys);
            assert_eq!(m, decrypted);
            let parsed = bigint_to_string(&decrypted);
            assert_eq!(message, parsed);
        }
    }
}
