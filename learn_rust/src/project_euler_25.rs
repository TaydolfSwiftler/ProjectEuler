use dashu_int;
use dashu_int::UBig;
// Frist Fibonacci Number containing more than 1000 digits

//Recursive approach
pub fn fib_rec(n: UBig) -> UBig {
    if n == UBig::ONE {
        return UBig::ONE;
    }
    return n.clone() + fib_rec(n - UBig::ONE);
}

//Overflows Stack for big n

pub fn fib(n: UBig) -> UBig{
    if n < UBig::ZERO {
        panic!("{} is negative!", n);
    } else if n == UBig::ZERO {
        panic!("zero is not a right argument to fibonacci()!");
    } else if n == UBig::ONE {
        return UBig::ONE;
    }

    let mut sum: UBig = UBig::ZERO;
    let mut last: UBig = UBig::ZERO;
    let mut curr: UBig = UBig::ONE;
    let mut i = UBig::ZERO;
    while i < n {
        sum = last + curr.clone();
        last = curr;
        curr = sum.clone();
        i = i + UBig::ONE;
    }
    sum
}

pub fn first_1000_digit_fib() -> UBig {



    UBig::ONE
}


fn length(n: u32, base: u32) -> u32 {
    let mut power = base;
    let mut count = 1;
    while n >= power {
        count += 1;
        if let Some(new_power) = power.checked_mul(base) {
            power = new_power;
        } else {
            break;
        }
    }
    count
}