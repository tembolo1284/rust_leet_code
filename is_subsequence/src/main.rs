/*Given two strings s and t, return true if s is a subsequence of t, or false otherwise.

A subsequence of a string is a new string that is formed from the original string by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (i.e., "ace" is a subsequence of "abcde" while "aec" is not).*/

pub fn is_subsequence(s: String, t: String) -> bool {
    let mut iter = t.chars();
    for c in s.chars() {
        match iter.find(|&p| p == c) {
            Some(_) => (),
            None => return false,
        }
    }
    true
}

fn main() {
    let s1 = "abc".to_string();
    let s2 = "axc".to_string();
    let t1 = "ahbgdc".to_string();
    let t2 = t1.clone();
    let result1 = is_subsequence(s1, t1);
    let result2 = is_subsequence(s2, t2);
    println!("result1 is: {}", result1);
    println!("result2 is: {}", result2);
}
