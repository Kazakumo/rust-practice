use crate::hash_map_practice::integer_mean_median_mode::integer_mean_median_mode;

pub mod hash_map_practice;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let mut v2 = vec![1, 2, 3, 3, 2, 1, 4, 5, 7, 4, 3];

    hash_map_practice::new_hash_map::new_hash_map();
    integer_mean_median_mode(&mut v2)
}
