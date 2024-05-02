// There is an extremely simple solution without coding at all
// Factor all Numbers 2 - 20 in primes and eleminate the duplicates you dont' need
// you need to keep 4 2s for 16 for example
// answer 232792560

use crate::project_euler_3::prime_factors;

pub fn smallest_multiple(cap: u64) -> u64 {
    let mut needed_factors: Vec<u64> = vec![];
    let mut needed_factors_temp: Vec<u64> = vec![];
    for i in 2..cap+1 {
        let mut fac = prime_factors(i);
        let mut fac_copy = fac.copy();
        let mut fac_sort = fac.sort();
        let mut fac_set = fac_sort.dedup();
        for j in fac_set {
            let freq_in_main = needed_factors.iter().filter(|&n| *n == j).count();
            if freq_in_main == 0 {
                continue;
            } else {
                for k in 0..needed_factors.len() {
                    if needed_factors[k] == j {
                        needed_factors = needed_factors.remove(k);
                    }
                }

            }
                // let freq_in_side = fac_copy.iter().filter(|&n| *n == j).count();
                // let diff:i32 = freq_in_main - freq_in_side;
                // if diff > 0 {

                //}

            }
        }
    }
    3
}
