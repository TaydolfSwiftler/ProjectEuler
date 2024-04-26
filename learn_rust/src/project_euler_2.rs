pub fn even_fib_sum_below_fmax(f_max: u32) -> u32 {
    let mut f_n_minus1: u32 = 1;
    let mut f_n: u32 = 2;
    let mut sum: u32 = 2;
    let mut dummy: u32;

    while f_n < f_max {
        dummy = f_n;
        f_n = f_n + f_n_minus1;
        f_n_minus1 = dummy;
        if f_n % 2 == 0 {
            sum = sum + f_n;
        }
    }
    sum
}
// runs in 51 microseconds