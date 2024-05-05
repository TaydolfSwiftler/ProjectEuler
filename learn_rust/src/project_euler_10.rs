use crate::project_euler_7::actually_good_sieve;

pub fn prime_sum_below_cap(cap: usize) -> usize {
    let primes = actually_good_sieve(cap);
    let mut i = 0;
    let mut sum: usize = 0;
    while i < primes.len() {
        if primes[i] {
            sum += i;
        }
        i += 1;
    }
    sum
}

// returns answer of 142913828922 in 32.3 ms