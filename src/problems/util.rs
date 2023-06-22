pub fn next_prime(i: u64, m: &u64) -> Option<u64> {
    for a in i + 1..=*m {
        if is_prime(a) {
            return Some(a);
        }
    }
    None // No prime number found inside the range
}

pub fn is_prime(i: u64) -> bool {
    for a in 2..i / 2 {
        if i % a == 0 {
            // Was able to divide it by a number, not prime
            return false;
        }
    }

    return true; // It is a prime number
}
