/// Problem 3
/// Find the largest prime factor
/// ex; largest prime factor for 13195 = 29
pub fn solve(mut m: u64) -> u64 {
    let mut out: u64 = 0;
    let mut a = 2;

    while a < m {
        a = match next_prime(a, &m) {
            Some(x) => x,
            None => break,
        };

        if m % a == 0 {
            m /= a;
            out = a; // Set new biggest prime factor
        }
    }

    out
}

fn next_prime(i: u64, m: &u64) -> Option<u64> {
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
