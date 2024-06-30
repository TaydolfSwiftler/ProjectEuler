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
