fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();
    for i in s.chars() {
        match i {
            '{' => stack.push('}'),
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '}' | ')' | ']' if Some(i) != stack.pop() => return false,
            _ => (),
        }
    }
    return stack.is_empty();
}

fn main() {
    let test1: String = "()".to_string();
		let test2: String = "{}(/".to_string();
    println!("Is test1 valid? {}", is_valid(test1));
    println!("Is test2 valid? {}", is_valid(test2));
}
