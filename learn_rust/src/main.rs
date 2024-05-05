mod project_euler_7;
mod project_euler_10;
mod project_euler_12;
mod project_euler_14;

use std::time::Instant;
use crate::project_euler_10::prime_sum_below_cap;
use crate::project_euler_12::divisible_trig_nums;


fn main() {
    let start = Instant::now();

    println!("{:?}",divisible_trig_nums(1000));

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}


