/*Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.

You must write an algorithm with O(log n) runtime complexity.*/

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    for (i, v) in nums.iter().enumerate() {
        if target <= *v {
            return i as i32;
        }
    }
    nums.len() as i32
}

fn main() {
    let v1: Vec<i32> = vec![1, 2, 4, 7, 8, 11, 14, 22];
    let v2: Vec<i32> = vec![-2, -1, 2, 4, 7, 8, 11, 14, 22];

    let target1: i32 = 9;
		let target2: i32 = 0;
    let result1 = search_insert(v1, target1);
    let result2 = search_insert(v2, target2);
    println!("result1 index: {}", result1);
    println!("result2 index: {}", result2);
}
