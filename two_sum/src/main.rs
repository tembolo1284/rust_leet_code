/*Given an array of integers nums and an integer target, return indices of the two numbers
such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the
same element twice.

You can return the answer in any order.
*/

use std::collections::HashMap;


    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_to_idx: HashMap<i32, i32> = HashMap::new();

        for (idx, num) in nums.iter().enumerate() {
            match num_to_idx.get(&(target - *num)) {
                Some(&idx2) => return vec![idx as i32, idx2],
                None => num_to_idx.insert(*num, idx as i32),
            };
        }
        vec![]
    }


fn main() {
    let vec1 = vec![3, 2, 7, 1, 5, 4];
    let target = 6;
    let output = two_sum(vec1, target);

    for i in 0..output.len() {
        println!("{}", output[i]);
    }
}
