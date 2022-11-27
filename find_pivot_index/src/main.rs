/*Given an array of integers nums, calculate the pivot index of this array.

The pivot index is the index where the sum of all the numbers strictly to the left of the index is equal to
the sum of all the numbers strictly to the index's right.

If the index is on the left edge of the array, then the left sum is 0 because there are no elements to the left.
This also applies to the right edge of the array.

Return the leftmost pivot index. If no such index exists, return -1.*/

pub fn pivot_index(nums: Vec<i32>) -> i32 {
    match nums.len() {
        0 => -1,
        1 => 0,
        _ => {
            let mut left_sum = 0;
            let mut right_sum = nums.iter().sum::<i32>();
            for i in 0..nums.len() {
                right_sum -= nums[i];
                if left_sum == right_sum {
                    return i as i32;
                }
                left_sum += nums[i];
            }
            -1
        }
    }
}

fn main() {
		let nums = vec![1,7,3,6,5,6];
		let result = pivot_index(nums);
		println!("result is: {}", result);
}
