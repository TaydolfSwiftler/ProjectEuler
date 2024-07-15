use std::collections::HashMap;

// https://www.geeksforgeeks.org/detect-cycle-in-a-graph/

pub fn proper_divisors_sum(input: u64) -> u64 {
    let mut result = vec![];
    for i in 1..(input / 2 + 1) {
        if input % i == 0 {
            result.push(i)
        }
    }
    result.iter().sum()
}

pub fn proper_divisors_sum_f(input: u64) -> u64 {
    let mut result = vec![1];
    for i in 2..(input as f32).sqrt() as u64 + 1 {
        if input % i == 0 {
            result.push(i);
            if input == i * i {
                break;
            }
            result.push(input / i);
        }
    }
    result.iter().sum()
}
// divisors < 100k 3s; fast divisors < 100k 37ms

pub fn get_social_chain(start: u64) -> Vec<u64> {
    let mut result = vec![start];
    let mut next = start;
    loop {
        next = proper_divisors_sum(next);
        if result.contains(&next) {
            break;
        }
        result.push(next);
    }
    result
}

pub fn find_amicable_chain(cap: u64) -> Vec<u64> {
    let mut div_sum_arr = vec![];
    let mut longest_chain_vec: Vec<u64> = vec![];
    for i in 0..cap {
        div_sum_arr.push(proper_divisors_sum_f(i));
    }
    let mut k = 0;
    loop {
        let j = div_sum_arr[k];
        let mut chain_vec = vec![];

        chain_vec.push(j);
        let mut runner = j.clone();

        loop {
            if runner >= cap {
                break;
            }

            runner = div_sum_arr[runner as usize];
            if chain_vec[0] == runner {
                if chain_vec.len() > longest_chain_vec.len() {
                    longest_chain_vec = chain_vec.clone();
                }
            }
            if chain_vec.contains(&runner) {
                break;
            }
            chain_vec.push(runner);
        }
        k += 1;
        if k >= cap as usize {
            break;
        }
    }
    longest_chain_vec
}

// This should obv. done with graph Theory, but meh its fast enough
// let lcv = find_amicable_chain(1_000_000);
// println!("Longest chain found: {:?}", lcv);
// let min = lcv.iter().min().unwrap_or(&0u64);
// println!("Smallest Entry: {}", min);
//Returns: Smallest Entry: 14316 in 1.0786446s
