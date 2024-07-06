// Area = sqrt(s(s-a)(s-b)(s-c)) s = ((a+b+c) / s

pub fn gen_alm_equ_trigs(perim_cap: u64) -> Vec<(u64, u64, u64, u64)> {
    let limit = ((perim_cap -1) / 3) as u64;

    let mut result = vec![];
    for i in 1..limit + 1 {
        let semi_perim = (2 * i + i + 1) / 2;
        let area = ((semi_perim * (semi_perim - i) * (semi_perim - i) * (semi_perim - i - 1)) as f64).powf(0.5);
        // println!("trig = {:?} \t\t semi_perim = {:?} \t\t area = {:?}", (i,i,i+1), semi_perim, area);
        if area % 1.0 == 0.0 {
            result.push((i, i, i + 1, area as u64))
        }
    }

    result
}

// TODO: Fix Overflow with dashu float