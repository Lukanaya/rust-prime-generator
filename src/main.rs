use num_bigint::{BigUint, RandBigInt, ToBigUint};
use num_traits::{Euclid, One};

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
 * If it is, then the number could be prime, if it isn't, then it's a compound
 */
fn miller_rabin_witness(n: &BigUint, a: u64) -> bool {
    let mut s = 0;
    let mut d: BigUint = n - BigUint::one();
    while d.clone() % &2u32 == BigUint::ZERO {
        s += 1;
        d = d.div_euclid(&2.to_biguint().unwrap());
    }
    let mut x: BigUint = BigUint::modpow(&a.to_biguint().unwrap(), &d, &n);
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

#[cfg(test)]
mod test {
    use num_bigint::{BigUint, ToBigUint};
    use num_traits::One;

    use crate::{generate_odd_n_bits, miller_rabin_witness};

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
        let bases = [2u64, 3, 5, 7];

        for &n in &carmichael_numbers {
            let n = n.to_biguint().unwrap();
            assert!(
                bases.iter().any(|&a| miller_rabin_witness(&n, a)),
                "No witness found for {}",
                n
            );
        }
    }
}
