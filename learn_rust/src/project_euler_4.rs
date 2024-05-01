pub fn pro_euler_4() -> u32 {
    //Generate all Products of 3 Digit Numbers+
    let mut pot_palindromes: Vec<u32> = vec![];
    let mut i = 100;
    let mut j = 100;
    while i < 1000 {
        j = 100;
        while j < i {
            pot_palindromes.push(i * j);
            j = j + 1;
        }
        i = i + 1;
    }
    let mut max = 0;
    for i in pot_palindromes {
        if palindrome_checker(i) && i > max {
            max = i;
        }
    }
    max
}

pub fn palindrome_checker(pot_pal: u32) -> bool{
    let mut reverse = 0;
    let mut forward = pot_pal;

    while forward > 0 {
        reverse = reverse * 10 + forward % 10;
        forward /= 10;
    }
    reverse == pot_pal
}

// To Do: Could be way fast if we start at the top (999) and try highst first and stop on the first hit
// runs 8.8 ms as is