use std::time::Instant;
mod project_euler_1;
mod project_euler_2;
mod project_euler_3;

use crate::project_euler_1::multiples_of_3_or_5;
use crate::project_euler_2::even_fib_sum_below_fmax;
use crate::project_euler_3::{prime_factors, print_vec};

fn main() {
    let start = Instant::now();
    //println!("The answer for problem 1 is: {}", multiples_of_3_or_5(1000));
    //println!("The answer for problem 2 is: {}", even_fib_sum_below_fmax(4_000_000));
    //println!("The answer for problem 3 is: {}", prime_factors(10)[0]);
    print_vec(prime_factors(600851475143));
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
