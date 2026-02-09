pub fn factors(n: u64) -> Vec<u64> {
    let mut x = n;
    
    let mut factors = Vec::new();
    let mut num = 2;

    while x > 1 {
        if x.is_multiple_of(num) {
            factors.push(num);
            x /= num;
        } else {
            num += 1;
        }
    }

    factors
}

pub fn is_prime(n: u64) -> bool {
    if n.is_multiple_of(2) {
        return false;
    }

    let limit = (n as f64).sqrt() as u64;
    (3..=limit).step_by(2).all(|i| n % i != 0)
}

