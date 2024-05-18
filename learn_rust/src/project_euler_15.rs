// All path are length = 40 with 20 right and 20 down, how many permuations?
use dashu_int;
use dashu_int::UBig;

pub fn fac(input: usize) -> UBig {
    let mut result = UBig::ONE;
    let mut counter = UBig::from(input);
    loop {
        result = result * counter.clone();
        counter = counter - UBig::ONE;
        if counter == UBig::ZERO {
            break;
        }
    }
    result
}
pub fn nchr(top: usize, bottom: usize) -> UBig {
    if top == bottom {
        return UBig::ONE;
    }
    let result = fac(top) / (fac(bottom) * (fac(top - bottom)));
    result
}

//Solution nChr(40,20) = 137846528820 in 59µs
