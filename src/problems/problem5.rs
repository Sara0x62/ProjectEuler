/// Problem 5
/// Smallest multiple
/// 2520 is the smallest number that can be divided by the numbers 0 to 10 with 0 remainder
/// What is the smallest positive number dividable from 0 to 20 with 0 remainder
pub fn solve() -> u32 {
    let mut flag: bool = false;
    let mut out: u32 = 0;

    // Simple bruteforce solution - it slow tho
    for n in 1..u32::MAX {
        for div in 1..=20 {
            if n % div == 0 {
                flag = true;
            } else {
                flag = false;
                break;
            }
        }

        if flag {
            out = n;
            break;
        }
    }

    out
}
