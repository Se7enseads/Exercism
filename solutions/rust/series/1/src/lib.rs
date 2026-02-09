pub fn series(digits: &str, len: usize) -> Vec<String> {
    if digits.len() < len {return Vec::new();};

    let slice: Vec<char> = digits.chars().collect();
    
    slice.windows(len).map(|window| window.iter().collect()).collect()
}
