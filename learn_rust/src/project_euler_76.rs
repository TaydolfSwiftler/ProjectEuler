// It is possible to write five as a sum in exactly six different ways:
//How many different ways can one hundred be written as a sum of at least two positive integers?

//https://en.wikipedia.org/wiki/Integer_partition

pub fn pentagonal_numbers(n: u64) -> u64 {
    n * (3 * n - 1) / 2
}
