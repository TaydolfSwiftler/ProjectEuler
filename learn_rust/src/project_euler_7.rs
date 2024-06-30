pub fn prime_sieve(cap: usize) -> Vec<bool> {
    let mut sieve = vec![true; cap];
    sieve[0] = false;
    sieve[1] = false;
    for i in 2..cap / 2 {
        if sieve[i] {
            let mut runner = 2 * i;
            while runner < cap {
                sieve[runner] = false;
                runner += i;
            }
        }
    }
    sieve
}
// Sieve of 1_000_000_000 takes 10.3688467s seconds

const LIMIT: usize = 100000;
pub fn prime_sieve_const() -> [bool; LIMIT] {
    let mut sieve = [true; LIMIT];
    sieve[0] = false;
    sieve[1] = false;
    for i in 1..LIMIT / 2 {
        if sieve[i] {
            let mut runner = 2 * i;
            while runner < LIMIT {
                sieve[runner] = false;
                runner += i;
            }
        }
    }
    sieve
}

pub fn actually_good_sieve(n: usize) -> Vec<bool> {
    let mut sieve_array = vec![true; n + 1];
    sieve_array[0] = false;
    sieve_array[1] = false;
    for i in (4..n + 1).step_by(2) {
        sieve_array[i] = false;
    }

    let mut i = 3;
    while i * i <= n + 1 {
        if sieve_array[i] {
            for j in (i * i..n + 1).step_by(2 * i) {
                sieve_array[j] = false;
            }
        }
        i += 2;
    }
    return sieve_array;
}
