// All path are length = 40 with 20 right and 20 down, how many permuations?
use dashu_int;
use dashu_int::UBig;

pub fn fac (input: usize) -> UBig {
    let mut result = UBig::ONE;
    let mut counter = UBig::from(input);
    while true {
        result = result * counter.clone();
        counter = counter - UBig::ONE;
        if counter == UBig::ZERO {
            break;
        }
    }
    result
}
pub fn nchr (top: u128, bottom: u128) -> u128 {




    1
}