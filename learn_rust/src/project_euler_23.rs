use crate::project_euler_15;
use crate::project_euler_21::sum_of_proper_divisors;

pub fn is_abundant(input: usize) -> bool {
    if sum_of_proper_divisors(input) > input {
        return true;
    }
    false
}

pub fn sum_not_writabable_as_abundant(cap: usize) -> usize {
    let mut sum: usize = 0;
    let mut all_abundant_below_cap: Vec<usize> = vec![];
    for i in 1..cap {
        if is_abundant(i) {
            all_abundant_below_cap.push(i);
        }
    }
    println!("{:?}", all_abundant_below_cap);
    for i in 1..cap {
        sum += i;
        for j in &all_abundant_below_cap {
            if j >= &i {
                break;
            }
            if all_abundant_below_cap.contains(&(i - j)) {
                sum -= i;
                break;
            }
        }
    }
    sum
}

//This does work for the value of 28123, but is atrociously slow
//sum_not_writabable_as_abundant(28123) = 4179871 in 146 seconds
//ToDo: figure something out yo
