/// Problem 1 - Project Euler
/// Multiples of 3 or 5
///
/// Find the sum of all multiples of 3 or 5 below 1000
pub fn solve(max: u32) -> u32 {
    let mut sum: u32 = 0;
    for i in 0..max {
        if (i % 3 == 0) || (i % 5 == 0) {
            sum += i;
        }
    }

    sum
}
