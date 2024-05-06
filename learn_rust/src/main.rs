use std::time::Instant;
use dashu_int;
use dashu_int::UBig;
mod project_euler_15;
use crate::project_euler_15::fac;

fn main() {
    let start = Instant::now();

    println!("{}", fac(50000));


    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}


