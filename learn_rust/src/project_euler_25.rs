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

pub fn fib(n: UBig) -> UBig {
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

pub fn first_n_digit_fib(n: usize) -> UBig {
    let mut sum: UBig;
    let mut last: UBig = UBig::ZERO;
    let mut curr: UBig = UBig::ONE;
    let mut i = UBig::ZERO;
    loop {
        sum = last + &curr;
        last = curr;
        curr = sum.clone();
        i = i + UBig::ONE;
        if count_digits(sum) >= n {
            return i + UBig::ONE;
        }
    }
}

//first_n_digit_fib(1000) returns 4782 after  61.2955ms

pub fn count_digits(input: UBig) -> usize {
    let mut base = UBig::from(10u8);
    let mut count = 1;
    while base <= input {
        count += 1;
        base = base * UBig::from(10u8);
    }
    count
}
