use std::fmt::Display;

use num_bigint::{BigInt, BigUint, RandBigInt, ToBigInt, ToBigUint};
use num_traits::{Euclid, One};

fn main() {
    let keys  = generate_keys(4096);
    println!("{} \n {}", keys.0, keys.1);
}

pub struct PrivateKey{
    p: BigUint,
    q: BigUint,
    d: BigUint,
}

impl Display for PrivateKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.p, self.q, self.d)
    }
}

pub struct PublicKey {
    n: BigUint,
    e: BigUint,
}

impl Display for PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.n, self.e)
    }
}

/*
 * Generates an odd number with a length of n bits
 */
fn generate_odd_n_bits(n: &u64) -> BigUint {
    let mut rng = rand::thread_rng();
    let mut nombre = rng.gen_biguint(n - 1);
    nombre <<= 1;
    nombre | BigUint::one()
}

/*
 * Checks if the number 'n' is a Miller Rabin witness to 'a'
 * If it is, then the number isn't prime, if it isn't then the number could be prime
 */
fn miller_rabin_witness(n: &BigUint, a: &BigUint) -> bool {
    let mut s = 0;
    let mut d: BigUint = n - BigUint::one();
    while d.clone() % 2u32 == BigUint::ZERO {
        s += 1;
        d = d.div_euclid(&2.to_biguint().unwrap());
    }
    let mut x: BigUint = BigUint::modpow(a, &d, n);
    if x == BigUint::one() || x == n - BigUint::one() {
        return false;
    }
    for _ in 0..s - 1 {
        x = BigUint::modpow(&x, &2.to_biguint().unwrap(), n);
        if x == n - BigUint::one() {
            return false;
        }
    }
    true
}

/**
 * Checks if a given number is prime using the Miller Rabin test
 */
fn miller_rabin_test(n: &BigUint) -> bool {
    let nb_tests = 40;
    let mut rng = rand::thread_rng();
    for _ in 0..nb_tests {
        let two = 2.to_biguint().unwrap();
        let a = rng.gen_biguint_range(&two, &(n - BigUint::one()));
        if miller_rabin_witness(n, &a){
            return false
        }
    }
    true
}

fn generate_prime_n_bits(size: u64) -> BigUint{
    let mut number = generate_odd_n_bits(&size);
    while !miller_rabin_test(&number) {
        number = generate_odd_n_bits(&size);
    }
    number 
}

/* 
 * Generate a tuple of keys this way : (<private key>, <public key>)
 * Where private key is : (p,q,d) (BitUint, BigUint, BigUint)
 * and public key is (n,e) (BigUint, BigUint)
 */
fn generate_keys(n: u64) -> (PrivateKey, PublicKey) {
    let p = generate_prime_n_bits(n);
    let q = generate_prime_n_bits(n);
    let n = &p*&q;
    let pub_exp: BigInt = 65537.to_bigint().unwrap();
    let phi = (&p-BigUint::one()) * (&q-BigUint::one());

    let d = pub_exp.modpow(&(BigInt::ZERO - BigInt::one()), &phi.to_bigint().unwrap()).to_biguint().unwrap();
    let private_key = PrivateKey{
        p,
        q,
        d,
    };
    let public_key = PublicKey {
        n,
        e: pub_exp.to_biguint().unwrap(),
    };
    (private_key, public_key)
}

#[cfg(test)]
mod test {
    use num_bigint::{BigUint, ToBigUint};
    use num_traits::One;

    use crate::{generate_odd_n_bits, miller_rabin_test, miller_rabin_witness};

    #[test]
    fn test_odd_numbers() {
        for _ in 0..100 {
            let x = generate_odd_n_bits(&512);
            let rem = x % 2.to_biguint().unwrap();
            assert_eq!(rem, BigUint::one());
        }
    }

    #[test]
    fn test_miller_rabin_witness() {
        let carmichael_numbers = [561u64, 1105, 1729, 2465, 2821, 6601, 8911];
        let bases: Vec<BigUint> = [2u64, 3, 5, 7, 11, 13, 17]
            .iter()
            .map(|&x| x.to_biguint().unwrap())
            .collect();

        for &n in &carmichael_numbers {
            let n = n.to_biguint().unwrap();
            assert!(
                bases.iter().any(|a| miller_rabin_witness(&n, a)),
                "No witness found for {}",
                n
            );
        }
    }

    #[test]
    fn test_miller_rabin_test(){
        let carmichael_numbers = [561u64, 1105, 1729, 2465, 2821, 6601, 8911];
        let known_primes = [983u64, 409, 127, 199, 743, 617, 907, 17, 997, 1013];

        for &n in &carmichael_numbers{
            let n = n.to_biguint().unwrap();
            assert_eq!(miller_rabin_test(&n), false);
        }

        for &n in &known_primes{
            let n = n.to_biguint().unwrap();
            assert_eq!(miller_rabin_test(&n), true);
        }
    }
}
