/*You are climbing a staircase. It takes n steps to reach the top.

Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?*/
pub fn climb_stairs(n: i32) -> i32 {
    let mut vec1 = vec![1, 2];
		let n = n as usize;

    for i in 2..n {
        vec1.push(vec1[i - 1] + vec1[i - 2]);
    }
    vec1[n - 1]
}

fn main() {
    let input1: i32 = 5;
    let input2: i32 = 11;
    let result1 = climb_stairs(input1);
    let result2 = climb_stairs(input2);
    println!("result1 = {}", result1);
    println!("result2 = {}", result2);
}
