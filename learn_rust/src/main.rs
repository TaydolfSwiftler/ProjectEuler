use std::time::Instant;
mod project_euler_1;
mod project_euler_2;
mod project_euler_3;
mod project_euler_4;
mod project_euler_6;

use crate::project_euler_1::multiples_of_3_or_5;
use crate::project_euler_2::even_fib_sum_below_fmax;
use crate::project_euler_3::{prime_factors, print_vec};
use crate::project_euler_4::{palindrome_checker, pro_euler_4};
use crate::project_euler_6::sum_square_diff;

fn main() {
    let start = Instant::now();
    //println!("The answer for problem 1 is: {}", multiples_of_3_or_5(1000));
    //println!("The answer for problem 2 is: {}", even_fib_sum_below_fmax(4_000_000));
    //print_vec(prime_factors(600851475143)); // Answer to Problem 3
    //println!("The answer for problem 4 is: {}", pro_euler_4());
    println!("The anser vor problem 6 is: {}", sum_square_diff(25000));
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
