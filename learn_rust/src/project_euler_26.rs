use dashu_float;
use dashu_float::DBig;
use std::dbg;
use std::iter::repeat;

pub fn reciprocal_cycles(cap: usize, precision: usize) -> usize {
    let zeros = std::iter::repeat("0").take(precision).collect::<String>();
    for i in 2..cap {
        let mut num_string = i.to_string();
        num_string.push_str(".");
        num_string.push_str(&zeros);
        let f1 = DBig::ONE;
        let f2 = DBig::from_str_native(&num_string).unwrap();
        println!("{}", f1 / f2);
        let s: &str = "0.23475089234";
        println!("{}", s.chars().nth(3).unwrap());
    }
    1
}

fn get_rec_cycle(input: &DBig) -> usize {
    let mut input_str = input.to_string();
    let mut rec: &str;
    let mut i = 2;
    let mut flag = true;
    while i < input_str.len() {
        if flag && input_str.chars().nth(i).unwrap().as_bytes() == "0".as_bytes() {
            i += 1;
            continue;
        } else {
            flag = false;
        }



    }

    1
}
