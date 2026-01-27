use num_bigint::{BigUint, RandBigInt, ToBigUint};
use num_traits::{Euclid, One};
use rand::Rng;

fn main() {}

/*
 * Generates an odd number with a length of n bits
 */
fn generate_odd_n_bits(n: u64) -> BigUint {
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
    while d.clone() % &2u32 == BigUint::ZERO {
        s += 1;
        d = d.div_euclid(&2.to_biguint().unwrap());
    }
    let mut x: BigUint = BigUint::modpow(&a, &d, &n);
    if x == BigUint::one() || x == n - BigUint::one() {
        return false;
    }
    for _ in 0..s.clone() - 1 {
        x = BigUint::modpow(&x, &2.to_biguint().unwrap(), &n);
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
        if miller_rabin_witness(&n, &a){
            return false
        }
    }
    true
}

#[cfg(test)]
mod test {
    use num_bigint::{BigUint, ToBigUint};
    use num_traits::One;

    use crate::{generate_odd_n_bits, miller_rabin_test, miller_rabin_witness};

    #[test]
    fn test_odd_numbers() {
        for _ in 0..100 {
            let x = generate_odd_n_bits(512);
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
