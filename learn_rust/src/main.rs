mod project_euler_8;

use std::time::Instant;
use crate::project_euler_8::largest_product;


fn main() {
    let start = Instant::now();

    println!("{}", largest_product());

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}


