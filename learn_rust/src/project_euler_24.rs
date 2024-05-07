use itertools;
use itertools::Itertools;

pub fn create_all_iterations(it_basis: Vec<usize>) -> usize {
    let mut perm_vec_len = 0;
    let perm_vec = it_basis.iter().permutations(it_basis.len()).unique();
    for i in perm_vec {
        perm_vec_len += 1;
        if perm_vec_len == 1_000_000 {
            println!("{:?}", i);
        }
    }
    perm_vec_len
}

// create_all_iterations(vec![0,1,2,3,4,5,6,7,8,9]) returns[2, 7, 8, 3, 9, 1, 5, 4, 6, 0] in 4.3s
//This is beyond stupid, but i dont understand those itertools objects
//i check a million time if it's the millionth element lol
//ToDo: hardcode the generation and stop after a million