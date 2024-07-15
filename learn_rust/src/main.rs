#![allow(unused_imports)]
#![allow(dead_code)]

use std::collections::HashMap;
use std::num;
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
mod project_euler_70;
mod project_euler_75;
mod project_euler_76;
mod project_euler_94;
mod project_euler_95;
mod scribbles;
mod project_euler_100;

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
use crate::project_euler_61::{
    are_cyclic, find_cycles, four_digit_fig_nums, get_bounds, nth_oct_number, nth_trig_number,
};
use crate::project_euler_7::{actually_good_sieve, prime_sieve};
use crate::project_euler_70::{gen_totient_vec, gen_unique_prime_facs, is_permutation, solve_61};
use crate::project_euler_75::{
    gcd, gen_primitive_pyt_trips, gen_some_trigs, singular_integer_right_trigs,
};
use crate::project_euler_76::{plus_minus_part, plus_minus_part_diffs};
use crate::project_euler_94::gen_alm_equ_trigs;
use crate::project_euler_95::{
    find_amicable_chain, get_social_chain, proper_divisors_sum, proper_divisors_sum_f,
};
use crate::scribbles::get_rand_vec;

fn main() {
    let start = Instant::now();

    let diffs = plus_minus_part_diffs(100);
    println!("{:?}", diffs);

    let plus_minus_vec = plus_minus_part(&diffs);
    println!("{:?}", plus_minus_vec);

    println!("Time eslapsed: {:?}", start.elapsed());

}
