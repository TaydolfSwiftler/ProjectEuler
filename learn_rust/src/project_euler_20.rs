use dashu_int;
use dashu_int::UBig;

pub fn digit_sum(input: UBig) -> UBig {
    let mut sum = UBig::ZERO;
    let mut input_var = input.clone();
    loop {
        sum += input_var.clone() % UBig::from(10u32);
        input_var = input_var / UBig::from(10u32);
        if input_var == UBig::ZERO {
            break;
        }
    }
    sum
}

// digit_sum(fac(100)) finds 648 in 91.2µs
