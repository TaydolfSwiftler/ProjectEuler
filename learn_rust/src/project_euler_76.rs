// It is possible to write five as a sum in exactly six different ways:
//How many different ways can one hundred be written as a sum of at least two positive integers?
//YT: Mathologer - The hardest hwat comes next

//https://en.wikipedia.org/wiki/Integer_partition

// First create the list of numbers that give the index of +/-

pub fn plus_minus_part_diffs(cap: u64) -> Vec<u64> {
    let mut result_vec = vec![];
    let mut naturals: u64 = 1;
    let mut odds: u64 = 3;
    while result_vec.len() <= cap as usize {
        result_vec.push(naturals);
        result_vec.push(odds);
        naturals += 1;
        odds += 2;
    }
    result_vec
}

pub fn plus_minus_part(diff: &Vec<u64>) -> Vec<u64> {
    let mut result_vec = vec![1];
    let mut index = 0;
    while index < diff.len() {
        result_vec.push(result_vec[index] + diff[index]);
        index += 1;
    }
    result_vec
}

pub fn partition_numbers(plus_minus_vec: Vec<u64>) -> Vec<u64> {
    let mut result_vec = vec![1, 1];
    //TODO: plus_minus_part gives the index of the next number to be added/subtracted 'Fibonacci style'
    // Pattern is ++,--,++,-- implementations should be easy


    result_vec
}