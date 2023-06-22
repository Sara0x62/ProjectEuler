use crate::util;
/// Problem 3
/// Find the largest prime factor
/// ex; largest prime factor for 13195 = 29
pub fn solve(mut m: u64) -> u64 {
    let mut out: u64 = 0;
    let mut a = 2;

    while a < m {
        a = match util::next_prime(a, &m) {
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