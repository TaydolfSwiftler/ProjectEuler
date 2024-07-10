// Area = sqrt(s(s-a)(s-b)(s-c)) s = ((a+b+c) / s

pub fn gen_alm_equ_trigs(perim_cap: u64) -> u128 {
    let limit = ((perim_cap - 1) / 3) as u64;

    let mut result = vec![];
    for i in 1..limit + 1 {
        let semi_perim = ((2 * i + i + 1) / 2) as f64;
        let area = ((semi_perim
            * (semi_perim - i as f64)
            * (semi_perim - i as f64)
            * (semi_perim - i as f64 - 1.0)) as f64)
            .powf(0.5);
        println!(
            "trig = {:?} \t\t semi_perim = {:?} \t\t area = {:?}",
            (i, i, i + 1),
            semi_perim,
            area
        );
        if area % 1.0 == 0.0 {
            result.push((i, i, i + 1, area as u64))
        }
    }
    let mut sum: u128 = 0;
    for i in result {
        sum += i.0 as u128 + i.1 as u128 + i.2 as u128;
    }

    sum
}

// TODO: Fix Inprecision with dashu float or new approach
