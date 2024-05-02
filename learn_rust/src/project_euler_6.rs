pub fn sum_square_diff(input: u64) -> u64 {
    let mut sum_squares_before: u64 = 0;
    let mut sum_squares_after: u64 = 0;
    for i in 1..input + 1{
        sum_squares_before += i * i
    }
    for j in 1..input + 1{
        sum_squares_after += j;
    }
    sum_squares_after = sum_squares_after * sum_squares_after;
    sum_squares_after - sum_squares_before
}