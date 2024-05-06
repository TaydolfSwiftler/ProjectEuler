use std::time::Instant;
use std::collections::HashMap;

use dashu_int;
use dashu_int::UBig;
mod project_euler_15;
mod project_euler_16;
mod project_euler_20;
mod project_euler_21;

use crate::project_euler_15::{fac, nchr};
use crate::project_euler_16::power_digit_sum;
use crate::project_euler_20::digit_sum;
use crate::project_euler_21::{sum_of_amicable_numbers_bwlow_cap, sum_of_proper_divisors};

fn main() {
    let start = Instant::now();

    println!("{}", sum_of_amicable_numbers_bwlow_cap(10000));

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}


