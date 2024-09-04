// TODO: Implement Lucas Lehmer Mersenne Primality Test
// TODO: Try to Multithread it
// num_prime for sanity checking and benchmarking

use num_prime::{PrimalityTestConfig, FactorizationConfig};
use num_prime::nt_funcs::{is_prime, factorize, factors, primes};
use num_prime::buffer::NaiveBuffer;
use dashu_int::UBig;

pub fn use_is_prime_t(c: u128) -> bool {
    is_prime(&c, None).probably()
}

pub fn professional_sieve(limit: u64) -> Vec<u64> {
    primes(limit)
}

pub fn lucas_lehmer(mersenne_power: usize) -> bool {
    let mut s = UBig::from(4u8);
    let m = UBig::from(2u8).pow(mersenne_power) - UBig::ONE;
    for _i in 0..(mersenne_power -2) {
        s = (s.pow(2) - UBig::from(2u8)) % &m;
    }
    if s == UBig::ZERO {
        true
    }
    else {
        false
    }
}
