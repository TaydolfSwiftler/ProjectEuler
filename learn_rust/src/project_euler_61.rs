use itertools::Itertools;
use std::hint::black_box;

pub fn nth_trig_number(nth: u64) -> u64 {
    nth * (nth + 1) / 2
}

pub fn nth_sqaure_number(nth: u64) -> u64 {
    nth * nth
}

pub fn nth_pent_number(nth: u64) -> u64 {
    nth * (3 * nth - 1) / 2
}

pub fn nth_hex_number(nth: u64) -> u64 {
    nth * (2 * nth - 1)
}

pub fn nth_hept_number(nth: u64) -> u64 {
    nth * (5 * nth - 3) / 2
}

pub fn nth_oct_number(nth: u64) -> u64 {
    nth * (3 * nth - 2)
}

pub fn fig_numbers(nth: u64, count: usize) -> u64 {
    return match count {
        0 => nth_trig_number(nth),
        1 => nth_sqaure_number(nth),
        2 => nth_pent_number(nth),
        3 => nth_hex_number(nth),
        4 => nth_hept_number(nth),
        5 => nth_oct_number(nth),
        _ => 0,
    };
}
pub fn get_bounds(f: fn(u64) -> u64) -> (u64, u64) {
    //This is inefficient, why calc thgem twice, once only for bounds
    let mut i = 1;
    let min;
    let max;
    loop {
        if f(i) >= 1000 {
            min = i;
            break;
        }
        i += 1;
    }
    loop {
        if f(i) >= 10000 {
            max = i - 1;
            break;
        }
        i += 1;
    }
    (min, max)
}
pub fn four_digit_fig_nums() -> Vec<Vec<u64>> {
    let mut fig_nums: Vec<Vec<u64>> = vec![vec![]; 6];
    let trig_bounds = get_bounds(nth_trig_number);
    let sqar_bounds = get_bounds(nth_sqaure_number);
    let pent_bounds = get_bounds(nth_pent_number);
    let hex_bounds = get_bounds(nth_hex_number);
    let hept_bounds = get_bounds(nth_hept_number);
    let oct_bounds = get_bounds(nth_oct_number);
    let bounds = vec![
        trig_bounds,
        sqar_bounds,
        pent_bounds,
        hex_bounds,
        hept_bounds,
        oct_bounds,
    ];

    for i in 0..6 {
        for j in bounds[i].0..bounds[i].1 {
            fig_nums[i].push(fig_numbers(j, i))
        }
    }
    fig_nums
}

pub fn are_cyclic(num1: u64, num2: u64) -> bool {
    let str1 = num1.to_string();
    let str2 = num2.to_string();
    if &str1[2..4] == &str2[0..2] {
        return true;
    }
    false
}

pub fn find_cycles(fig_nums: Vec<Vec<u64>>) -> u64 {
    let mut pairs = vec![];
    for i in 0..6 {
        for j in 0..6 {
            pairs.push((i, j))
        }
    }
    pairs.retain(|&x| x.0 != x.1);

    let mut fig_pairs = vec![];

    for s in &pairs {
        for t in &fig_nums[s.0] {
            for u in &fig_nums[s.1] {
                if are_cyclic(*t, *u) {
                    fig_pairs.push((*t, *u, s.0, s.1))
                }
            }
        }
    }

    let mut figs = vec![];

    for a in &fig_pairs {
        for b in &fig_pairs {
            for c in &fig_pairs {
                if a.2 != b.2 && a.3 != b.3 && a.2 != b.3 && b.2 != a.3 && c.2 != a.2 && c.3 != a.3 && c.2 != a.3 && c.3 != a.2 && c.2 != b.2 && c.3 != b.3 && c.2 != b.3  && c.3 != b.2 {
                    if are_cyclic(a.1, b.0) && are_cyclic(b.1, c.0) && are_cyclic(c.1, a.0){
                        figs.push((a, b, c, "abc"))
                    }
                    if are_cyclic(b.1, a.0) && are_cyclic(a.1, c.0) && are_cyclic(c.1, b.0) {
                        figs.push((b, a, c, "bac"))
                    }
                }
            }
        }
    }

    figs[0].0.0 + figs[0].0.1 + figs[0].1.0 + figs[0].1.1 + figs[0].2.0 + figs[0].2.1
}

//TODO: Kill yourself
//println!("{:?}", find_cycles(four_digit_fig_nums())); runs in 3.2s and returns 28684