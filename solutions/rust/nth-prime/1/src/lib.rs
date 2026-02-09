pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2; // 0th prime is 2
    }

    let mut count = 1; // already counted prime 2
    let mut candidate = 3;

    while count <= n {
        if is_prime(candidate) {
            if count == n {
                return candidate;
            }
            count += 1;
        }
        candidate += 2; // skip even numbers
    }

    unreachable!() // should never reach here
}

// helper function to check if a number is prime
fn is_prime(num: u32) -> bool {
    if num < 2 {
        return false;
    }
    if num == 2 {
        return true;
    }
    if num % 2 == 0 {
        return false;
    }

    let sqrt = (num as f64).sqrt() as u32;
    for i in (3..=sqrt).step_by(2) {
        if num % i == 0 {
            return false;
        }
    }

    true
}

