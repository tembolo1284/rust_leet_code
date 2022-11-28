/*Given two strings s and t, determine if they are isomorphic.

Two strings s and t are isomorphic if the characters in s can be replaced to get t.

All occurrences of a character must be replaced with another character while preserving the order of 
characters. No two characters may map to the same character, but a character may map to itself.*/

pub fn is_isomorphic(s: String, t: String) -> bool {
    let mut assign_s_t = std::collections::BTreeMap::new();
    let mut assign_t_s = std::collections::BTreeMap::new();

    for (ch_s, ch_t) in s.chars().zip(t.chars()) {
        if *assign_s_t.entry(ch_s).or_insert(ch_t) != ch_t {
            return false;
        }
        if *assign_t_s.entry(ch_t).or_insert(ch_s) != ch_s {
            return false;
        }
    }

    true
}

fn main() {
		let s = "paper".to_string();
		let t = "title".to_string();
		let result = is_isomorphic(s, t);
		println!("result is: {}", result);
}
