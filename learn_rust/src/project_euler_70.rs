use crate::project_euler_7::actually_good_sieve;
use ::std::cmp::max;

//Find the value of n, 1 < n < 10^7 , for which phi(n) is a permutation of n and the ratio n/phi(n) produces a minimum.

//Step 1 Generate a shitload of primes
//actually good sieve returns bool Vector with true for primes

pub fn gen_unique_prime_facs(primes: Vec<bool>, cap: u32) -> Vec<Vec<u32>> {
    let mut prime_facs = vec![vec![]; 3];
    let mut runner: u32 = 3;
    let limit = max(cap, primes.len() as u32);
    while runner < limit {
        let mut n_facs = vec![];
        let mut i = 2;
        loop {
            if primes[i] {
                if runner % (i as u32) == 0 {
                    n_facs.push(i as u32)
                }
            }
            i += 1;
            if (i as u32) >= runner {
                prime_facs.push(n_facs);
                break;
            }
        }
        runner += 1;
    }
    prime_facs
}

pub fn gen_totient_vec(prime_facs: Vec<Vec<u32>>) -> Vec<u32> {
    let mut totient_vals = vec![0];
    let mut runner = 1;
    while runner < prime_facs.len() {
        if prime_facs[runner].is_empty() {
            totient_vals.push(runner as u32 - 1);
        } else {
            let mut val = runner.clone() as f32;
            for i in &prime_facs[runner] {
                val = val * (1f32 - (1f32 / *i as f32));
            }
            totient_vals.push(val.ceil() as u32);
        }
        runner += 1;
    }
    totient_vals
}

pub fn is_permutation(num1: u32, num2: u32) -> bool {
    let str1 = num1.to_string();
    let str2 = num2.to_string();
    let mut vec1: Vec<char> = str1.chars().collect();
    let mut vec2: Vec<char> = str2.chars().collect();
    vec1.sort_unstable();
    vec2.sort_unstable();
    if vec1 == vec2 {
        return true;
    }
    false
}

pub fn solve_61(totient_vals: Vec<u32>) -> (u32, u32) {
    let mut runner = 2;
    let mut candidates = vec![];
    while runner < totient_vals.len() {
        if is_permutation(runner as u32, totient_vals[runner]) {
            candidates.push((runner as u32, totient_vals[runner]))
        }
        runner += 1;
    }
    let mut min: f32 = 100000f32;
    let mut winner = (0, 0);
    for i in candidates {
        let ratio = i.0 as f32 / i.1 as f32;
        if ratio < min {
            min = ratio;
            winner = i;
        }
    }

    winner
}

// let primes = actually_good_sieve(10000000);
//
// println!("primes done : {:?}", start.elapsed());
//
// let primes_facs = gen_unique_prime_facs(primes, 10000000);
//
// println!("primes_fac_done : {:?}",start.elapsed());
//
// let totient_vals = gen_totient_vec(primes_facs);
//
// println!("totient vals done : {:?}",start.elapsed());
//
// println!("The winner is: {:?}", crate::project_euler_70::solve_61(totient_vals));

// primes done : 11.2485ms
// primes_fac_done : 41489.8356956s
// totient vals done : 41490.1173039s
// The winner is: (8319823, 8313928)
// Time elapsed: 41492.4267926s

//Appearently you only need to check semiprimes with resonable big primes facs, wouldnve been good to know
