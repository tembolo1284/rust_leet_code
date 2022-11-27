/*Given an integer numRows, return the first numRows of Pascal's triangle.

In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:*/

pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut res = vec![];
    for i in 0..num_rows as usize {
        // prefill the vector
        res.push(vec![1; i + 1]);
        for j in 1..i {
            res[i][j] = res[i - 1][j - 1] + res[i - 1][j];
        }
    }

    res
}

fn main() {
    let input = 10;
    let result = generate(input);

    println!("result: {:?}", result);
}
