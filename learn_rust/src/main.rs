mod project_euler_7;

use std::time::Instant;
use crate::project_euler_7::prime_sieve;

fn main() {
    let start = Instant::now();

    prime_sieve(10000000);

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
