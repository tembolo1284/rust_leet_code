/*Given a string s which consists of lowercase or uppercase letters, return the length of the longest
palindrome that can be built with those letters.

Letters are case sensitive, for example, "Aa" is not considered a palindrome here.*/
pub fn longest_palindrome(s: String) -> i32 {
    let mut d: [i32; 128] = [0; 128];
    for &b in s.as_bytes() {
        d[b as usize] += 1;
    }
    let mut answer = 0;
    for n in d.iter() {
        answer += n;
        if n % 2 != 0 {
            answer -= 1;
            if answer % 2 == 0 {
                answer += 1;
            }
        }
    }
    answer
}

fn main() {
		let s = "abccccdd".to_string();
		let result = longest_palindrome(s);
		println!("result: {:}", result);
}
