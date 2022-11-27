/*Given an array nums. We define a running sum of an array as runningSum[i] = sum(nums[0]…nums[i]).

Return the running sum of nums.*/

pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut result = nums.clone();
    for i in 1..nums.len() {
        result[i] += result[i - 1];
    }
    result
}

fn main() {
    let nums = vec![1, 2, 3, 4];

    let result = running_sum(nums);
    println!("result is: {:?}", result);
}
