pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut output = Vec::new();
    if digits.len() == len {
        return vec![digits.to_string()];
    }
    if digits.len() > len {
        for i in 0..digits.len() {
            let start = i;
            let end = start + len - 1;
            println!("{} {}", start, end);
            if end < digits.len() {
                output.push(digits[i..=end].to_string());
            } else if end >= digits.len() {
                break;
            }
        }
    }
    output
}
