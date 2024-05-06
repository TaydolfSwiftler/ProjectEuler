pub fn collatz_seq_len(mut input: usize) -> usize {
    let mut seq_len: usize = 1;
    while input != 1 {
        if input % 2 == 0 {
            input /= 2;
        }
        else {
            input = 3 * input + 1;
        }
        seq_len += 1;
    }
    seq_len
}

pub fn long_collatz_chain_blow_cap(cap: usize) -> (usize, usize) {
    let mut i = 1;
    let mut max_len = 0;
    let mut i_col_len;
    let mut input_max = 0;
    while i < cap {
        i_col_len = collatz_seq_len(i);
        if i_col_len > max_len {
            max_len = i_col_len;
            input_max = i;
        }
        i += 1;
    }
    (input_max, max_len)
}

//Solution (837799, 525) in 299.3724ms