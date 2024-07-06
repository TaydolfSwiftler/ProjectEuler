// https://projecteuler.net/problem=75
// https://en.wikipedia.org/wiki/Integer_triangle
// a^2 + b^2 = c^2

use multimap::MultiMap;
use std::collections::HashMap;

pub fn gen_some_trigs() -> MultiMap<u64, (f64, f64, f64)> {
    let mut trigs = vec![];
    let mut trigs_map = MultiMap::new();

    for a in 1..150000 {
        for b in a..150000 {
            let c = ((a as f64).powf(2.0) + (b as f64).powf(2.0)).powf(0.5);
            if c % 1.0 == 0.0 {
                trigs.push((a as f64, b as f64, c));
            }
        }
    }
    for i in trigs {
        trigs_map.insert((i.0 + i.1 + i.2) as u64, i);
    }
    trigs_map
}

// This should work but is just to slow >60seconds for values smaller 150000

// https://en.wikipedia.org/wiki/Formulas_for_generating_Pythagorean_triples

pub fn gcd(num1: u64, num2: u64) -> u64 {
    if num1 == 0 || num2 == 0 {
        return 0;
    }
    if num1 == num2 {
        return num1;
    }
    if num1 > num2 {
        return gcd(num1 - num2, num2);
    }
    gcd(num1, num2 - num1)
}

pub fn is_coprime(num1: u64, num2: u64) -> bool {
    if gcd(num1, num2) == 1 {
        return true;
    }
    false
}

pub fn gen_primitive_pyt_trips(limit: u64) -> Vec<(u64, u64, u64)> {
    let cap = (limit as f64).powf(0.5) as u64;
    let mut triples = vec![];
    for n in 1..cap {
        for m in n + 1..cap {
            if is_coprime(m, n) && ((m % 2 == 0 && n % 2 == 1) || (n % 2 == 0 && m % 2 != 0)) {
                let n_sq = n.pow(2);
                let m_sq = m.pow(2);
                triples.push((m_sq - n_sq, 2 * m * n, m_sq + n_sq));
            }
        }
    }
    triples
}

pub fn singular_integer_right_trigs(prim_trips: Vec<(u64, u64, u64)>, limit: u64) -> u64 {
    let mut count_vec = vec![0u8; limit as usize];

    for i in prim_trips {
        let mut runner_trip = i.clone();
        loop {
            let circumference = runner_trip.0 + runner_trip.1 + runner_trip.2;
            if circumference < limit {
                count_vec[circumference as usize] += 1;
                runner_trip.0 += i.0;
                runner_trip.1 += i.1;
                runner_trip.2 += i.2;
            } else {
                break;
            }
        }
    }
    let mut result_count: u64 = 0;
    for j in count_vec {
        if j == 1 {
            result_count += 1;
        }
    }
    result_count
}

//    let limit = 1_500_001;
//
//     let prim_triples = gen_primitive_pyt_trips(limit);
//     let solution_75 = singular_integer_right_trigs(prim_triples, limit);
//
//     println!("{:?}", solution_75);

// Gives 161667 in 33.5619ms
