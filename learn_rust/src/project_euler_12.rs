pub fn divisible_trig_nums (cap: usize) -> (usize, usize) {
    let mut trig_num: usize = 0;
    let mut i: usize = 0;
    let mut fac_num: usize;
    while true {
        i += 1;
        trig_num += i;
        fac_num = 1;
        for j in 1..(trig_num as f64).sqrt() as usize + 1 {
            if trig_num % j == 0 {
                fac_num += 1;
            }
            if fac_num * 2 - 1 > cap {
                return (trig_num, fac_num * 2 - 1)
            }
        }
    }
    (1,1)
}

//Extremely inefficient way to do this
// Solves for cap = 500: 76576500 with 501 factors in 184.7075ms