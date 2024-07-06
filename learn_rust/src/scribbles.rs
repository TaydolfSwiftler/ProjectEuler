use rand::prelude::*;

pub fn get_rand_vec(length: i64, low: i64, high: i64) -> Vec<i64> {
    let mut my_vec = vec![];
    for _i in 0..=length {
        push_element(&mut my_vec, low, high);
    }
    my_vec
}

fn push_element(input: &mut Vec<i64>, low: i64, high: i64) {
    let rand_int: i64 = rand::thread_rng().gen_range(low..(high + 1));
    input.push(rand_int);
}

pub fn divisors(n: u64) -> Vec<u64> {
    let mut result = vec![];
    for i in 1..(n / 2 + 1) {
        if n % i == 0 {
            result.push(i)
        }
    }
    result.push(n);
    result
}

//ROOTS AND POWERS
//let a:u64 = 3;
//let b:u64 = 5;
//let c = f64::powf((a.pow(2) + b.pow(2)) as f64, 0.5);

//HASHMAP
// let mut new_map = HashMap::new();
// new_map.insert(1,2);
// new_map.insert(5,2);
//
// for (key, value) in new_map {
//     println!("{}: {}", key, value);
// }
