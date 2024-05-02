pub fn prime_sieve(cap: usize) -> Vec<bool> {
    let mut sieve = vec![true; cap];
    sieve[0] = false;
    sieve[1] = false;
    for i in 2..cap / 2 {
        let mut runner =  2 * i;
        while runner < cap {
            sieve[runner] = false;
            runner += i;
        }
    }
    sieve
}