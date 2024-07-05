#![allow(unused_imports)]
#![allow(dead_code)]

use std::collections::HashMap;
use std::str::FromStr;
use std::time::Instant;

use dashu_float::{DBig, FBig};
use dashu_int;
use dashu_int::UBig;
mod project_euler_14;
mod project_euler_15;
mod project_euler_16;
mod project_euler_20;
mod project_euler_21;
mod project_euler_23;
mod project_euler_24;
mod project_euler_25;
mod project_euler_29;
mod project_euler_59;
mod project_euler_61;
mod project_euler_7;
mod scribbles;
mod project_euler_70;


use crate::project_euler_14::long_collatz_chain_blow_cap;
use crate::project_euler_15::{fac, nchr};
use crate::project_euler_16::power_digit_sum;
use crate::project_euler_20::digit_sum;
use crate::project_euler_21::{sum_of_amicable_numbers_bwlow_cap, sum_of_proper_divisors};
use crate::project_euler_23::{is_abundant, sum_not_writabable_as_abundant};
use crate::project_euler_24::create_all_iterations;
use crate::project_euler_25::{count_digits, fib, first_n_digit_fib};
use crate::project_euler_29::distinct_terms_in_powers;
use crate::project_euler_59::txt_file_parser;
use crate::project_euler_61::{are_cyclic, find_cycles, four_digit_fig_nums, get_bounds, nth_oct_number, nth_trig_number,
};
use crate::project_euler_7::{actually_good_sieve, prime_sieve};
use crate::project_euler_70::{gen_totient_vec, gen_unique_prime_facs, is_permutation, solve_61};
use crate::scribbles::get_rand_vec;

fn main() {
    let start = Instant::now();

    let primes = actually_good_sieve(10000000);

    println!("primes done : {:?}", start.elapsed());

    let primes_facs = gen_unique_prime_facs(primes, 10000000);

    println!("primes_fac_done : {:?}",start.elapsed());

    let totient_vals = gen_totient_vec(primes_facs);

    println!("totient vals done : {:?}",start.elapsed());

    println!("The winner is: {:?}", solve_61(totient_vals));

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
