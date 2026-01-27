use num_bigint::{BigUint, RandBigInt, ToBigUint};
use num_traits::{Euclid, One};

fn main() {
    
}

fn generer_entier_impair_n_bits(n: u64) -> BigUint {
    let mut rng = rand::thread_rng();
    let mut nombre = rng.gen_biguint(n-1);
    nombre <<= 1;
    nombre | BigUint::one()
}

fn temoin_miller(n: BigUint, a: u64) -> bool{
    let mut s = 0;
    let mut d: BigUint = n.clone()-BigUint::one();
    while d.clone()%&2u32 == BigUint::ZERO {
        s += 1;
        d = d.div_euclid(&2.to_biguint().unwrap());
    }
    let mut x: BigUint = BigUint::modpow(&a.to_biguint().unwrap(), &d, &n);
    if x == BigUint::one() || x == n.clone()-BigUint::one() {
        return false
    }
    for _ in 0..s.clone()-1 {
        x = BigUint::modpow(&x, &2.to_biguint().unwrap(), &n);
        if x == n.clone()-BigUint::one() {
            return false
        }
    }
    true
}