use std::time::Instant;
mod project_euler_1;
mod project_euler_2;
mod project_euler_3;
mod project_euler_4;
mod project_euler_5;

use crate::project_euler_1::multiples_of_3_or_5;
use crate::project_euler_2::even_fib_sum_below_fmax;
use crate::project_euler_3::{prime_factors, print_vec};
use crate::project_euler_4::{palindrome_checker, pro_euler_4};

fn main() {
    let start = Instant::now();
    //println!("The answer for problem 1 is: {}", multiples_of_3_or_5(1000));
    //println!("The answer for problem 2 is: {}", even_fib_sum_below_fmax(4_000_000));
    //print_vec(prime_factors(600851475143)); // Answer to Problem 3
    //println!("The answer for problem 4 is: {}", pro_euler_4());
    let mut a: Vec<u64> = vec![2,2,2,4,6,5,2,2,7,7,6];
    println!("{}", a.iter().filter(|&n| *n == 2).count());
    let index = a.iter().position(|x| *x == 2).unwrap();
    println!("index = {}", index);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
