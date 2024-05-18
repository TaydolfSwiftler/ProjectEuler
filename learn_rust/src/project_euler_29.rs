use dashu_int;
use dashu_int::UBig;
use std::collections::hash_set::HashSet;

//Need a set functionality
pub fn distinct_terms_in_powers(a: UBig, b: UBig) -> usize {
    let mut numbers_seen = HashSet::new();
    let mut i = UBig::from(2u8);
    let mut j: usize = 2;
    while i < a {
        while UBig::from(j) < b {
            numbers_seen.insert(i.pow(j));
            j += 1;
        }
        j = 2;
        i = i + UBig::ONE;
    }
    numbers_seen.len()
}

// distinct_terms_in_powers(UBig::from(101u8),UBig::from(101u8)) finds 9183 in 1.9862ms