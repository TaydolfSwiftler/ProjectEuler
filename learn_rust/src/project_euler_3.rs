pub fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = vec![];
    let mut i: u64 = 1;

    while i < n {
        i = i + 1;
        if (n as f64 / i as f64).fract() == 0.0 {
            factors.push(i);
            n = (n as f64 / i as f64) as u64;
            i = 1;
        }
    }
    factors
}

pub fn print_vec(vec: Vec<u64>) {
    println!("Vector length is {} and contains: ", vec.len());
    for i in vec {
        println!("{}", i);
    }
}
