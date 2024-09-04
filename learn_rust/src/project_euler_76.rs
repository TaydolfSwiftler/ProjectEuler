// It is possible to write five as a sum in exactly six different ways:
//How many different ways can one hundred be written as a sum of at least two positive integers?
//YT: Mathologer - The hardest hwat comes next

//https://en.wikipedia.org/wiki/Integer_partition

// First create the list of numbers that give the index of +/-

pub fn plus_minus_part_diffs(cap: u64) -> Vec<i32> {
    let mut result_vec = vec![];
    let mut naturals: i32 = 1;
    let mut odds: i32 = 3;
    while result_vec.len() <= cap as usize {
        result_vec.push(naturals);
        result_vec.push(odds);
        naturals += 1;
        odds += 2;
    }
    result_vec
}

pub fn plus_minus_part(diff: &Vec<i32>) -> Vec<i32> {
    let mut result_vec = vec![1];
    let mut index = 0;
    while index < diff.len() {
        result_vec.push(result_vec[index] + diff[index]);
        index += 1;
    }
    result_vec
}

pub fn partition_numbers(plus_minus_vec: Vec<i32>) -> Vec<i32> {
    let mut result_vec = vec![1, 1];
    let ones_vec: Vec<i32> = vec![1,1-1,-1,1,1,-1,-1,1,1,-1,-1,1,1,-1,-1];
    //TODO: plus_minus_part gives the index of the next number to be added/subtracted 'Fibonacci style'
    // Pattern is ++,--,++,-- implementations should be easy

    loop {
        let mut next_sum = 0;
        let mut runner = 0;

        while plus_minus_vec[runner] < result_vec.len() as i32 {
            if runner - plus_minus_vec[runner] as usize >= result_vec.len() {
                break;
            }
            next_sum += result_vec[runner - plus_minus_vec[runner] as usize ] * ones_vec[runner];
            runner += 1;
        }
        result_vec.push(next_sum);
        if result_vec.len() == 100 {
            break;
        }
    }


    result_vec
}

    // let diffs = plus_minus_part_diffs(100);
    //
    // let plus_minus_vec = plus_minus_part(&diffs);
    // println!("{:?}", plus_minus_vec);
    //
    // let partitions = partition_numbers(plus_minus_vec);
    // println!("{:?}", partitions);