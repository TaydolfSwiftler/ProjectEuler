use dashu_int;
use dashu_int::UBig;

pub fn power_digit_sum(base: usize, power: usize) -> UBig {
    let mut sum = UBig::ZERO;
    let base_big = UBig::from(base);
    let mut big_num = base_big.pow(power);
    loop {
        sum += big_num.clone() % UBig::from(10u32);
        big_num = (big_num / UBig::from(10u32));
        if big_num == UBig::ZERO {
            break;
        }
    }
    sum
}

//power_digit_sum(2,1000) = 1366 in 342 microseconds
