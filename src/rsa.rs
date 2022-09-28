use crate::exponentiation::binary;
use rug::{rand::RandState, Complete, Integer};
use std::ops::Add;

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

pub fn get_keys(p: &Integer, q: &Integer) -> KeyPair {
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

    // dbg!(&lambda_n);
    // dbg!((&x * &e).complete() % &lambda_n);
    // dbg!((&y * &e).complete() % &lambda_n);

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
    KeyPair { pubkey, privkey: d }
}

pub fn encrypt(m: &Integer, pubkey: &PublicKey) -> Integer {
    binary(m, &pubkey.e, Some(&pubkey.n))
}

pub fn decrypt(c: &Integer, keys: &KeyPair) -> Integer {
    binary(c, &keys.privkey, Some(&keys.pubkey.n))
}
