pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {return None;};
    
    let mut steps = 0;
    let mut x = n;
    
    while x != 1 {
        if x.is_multiple_of(2) {
            x /= 2
        } else {
            x = x * 3 + 1
        }
        steps += 1
    };
    
    Some(steps)
}
