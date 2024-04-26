pub fn multiples_of_3_or_5(ceil: i32) -> i32 {
    let mut i: i32 = 1;
    let mut sum: i32 = 0;
    while i < ceil {
        if i % 3 == 0 && i % 5 != 0 {
            sum = sum + i;
        }
        if i % 5 == 0 {
            sum = sum + i
        }
        i = i + 1;
    }
    sum
}
