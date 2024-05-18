use std::collections::HashMap;

pub fn sum_of_proper_divisors(input: usize) -> usize {
    let mut sum: usize = 0;
    for i in 1..input / 2 + 1 {
        if input % i == 0 {
            sum += i;
        }
    }
    sum
}

pub fn sum_of_amicable_numbers_bwlow_cap(cap: usize) -> usize {
    let mut sum: usize = 0;
    let mut divisor_sum_i;
    let mut amicable_pairs: HashMap<usize, usize> = HashMap::new();
    for i in 1..cap {
        if amicable_pairs.contains_key(&i) {
            continue;
        }
        divisor_sum_i = sum_of_proper_divisors(i);
        if sum_of_proper_divisors(divisor_sum_i) == i && divisor_sum_i != i {
            amicable_pairs.insert(i, divisor_sum_i);
            amicable_pairs.insert(divisor_sum_i, i);
        }
    }
    for i in amicable_pairs {
        sum += i.0;
        sum += i.1;
    }
    sum /= 2;
    sum
}

// This is dumb, because every pair is twice in the Hashmap, but i dont wanna fix it
// sum_of_amicable_numbers_bwlow_cap(10000) = 31626 in 50.0354ms
