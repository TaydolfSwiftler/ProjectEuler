pub fn pyth_triple_sum(abc_sum: u32) -> (u32, u32, u32) {
    let mut a: u32 = 1;
    let mut b: u32;
    let mut c: u32;
    while a < abc_sum {
        a += 1;
        b = 1;
        while a + b < abc_sum {
            b += 1;
            c = abc_sum - a - b;
            if a * a + b * b == c * c {
                return (a, b, c)
            }
        }
    }
    (1, 1, 1)
}
//finds solution in 465 mircroseconds