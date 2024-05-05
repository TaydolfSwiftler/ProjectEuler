use std::arch::x86_64::_mm_stream_si128;

//hello
pub fn largest_product() -> String {
    let mut str1 = "73167176531330624919225119674426574742355349194934
                        96983520312774506326239578318016984801869478851843
                        85861560789112949495459501737958331952853208805511
                        12540698747158523863050715693290963295227443043557
                        66896648950445244523161731856403098711121722383113
                        62229893423380308135336276614282806444486645238749
                        30358907296290491560440772390713810515859307960866
                        70172427121883998797908792274921901699720888093776
                        65727333001053367881220235421809751254540594752243
                        52584907711670556013604839586446706324415722155397
                        53697817977846174064955149290862569321978468622482
                        83972241375657056057490261407972968652414535100474
                        82166370484403199890008895243450658541227588666881
                        16427171479924442928230863465674813919123162824586
                        17866458359124566529476545682848912883142607690042
                        24219022671055626321111109370544217506941658960408
                        07198403850962455444362981230987879927244284909188
                        84580156166097919133875499200524063689912560717606
                        05886116467109405077541002256983155200055935729725
                        71636269561882670428252483600823257530420752963450";

    let mut str2 = remove_whitespace(str1);
    remove_zeroes(str1.to_string())
}

pub fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

fn remove_zeroes(mut str1: String) -> String {
    let mut i = 0;
    let mut index_low = 0;
    let mut index_high = 0;
    let mut to_be_cut_bumbers_vec: Vec<bool> = vec![false; str1.len()];
    while i < str1.len() {
        println!("{}", str1.chars().nth(i).unwrap());
        print_type_of(&str1.chars().nth(i).unwrap());
        if str1.chars().nth(i).unwrap() as u8 == "0".to_string().parse::<u8>().unwrap() {
            println!("penis");
            if i - 12 < 0 {
                index_low = 0;
            } else {
                index_low = i - 12;
            }
            if i + 12 > str1.len() - 1 {
                index_high = str1.len() - 1;
            } else {
                index_high = i + 12;
            }
            for j in index_low..index_high + 1 {
                to_be_cut_bumbers_vec[j] = true;
            }
        }
    i += 1;
    }
    let mut k = 0;
    let mut counter = 0;
    while k - counter < to_be_cut_bumbers_vec.len() {
        k = k - counter;
        if to_be_cut_bumbers_vec[k] {
            str1 = str1.chars().take(k).chain(str1.chars().skip(k)).collect();
            str1.remove(k);
            counter += 1;
        }
        k += 1;
    }
    str1
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}