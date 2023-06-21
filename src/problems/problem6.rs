/// Find the sum of the squares of the first n numbers
/// ex; (1^2 + 2^2 + 3^2 ... 10^2) = 385
///
/// and find the square of the sum of the first n numbers
/// ex; (1 + 2 + 3 ... + 10)^2 = 3025
///
/// return the difference -
/// 3025 - 385 = 2640
pub fn solve(n_nums: u32) -> u32 {
    let mut sum_sq: u32 = 0;
    let mut sq_sum: u32 = 0;

    for d in 1..=n_nums {
        sum_sq += d.pow(2);
        sq_sum += d;
    }
    sq_sum = sq_sum.pow(2);

    sq_sum - sum_sq
}
