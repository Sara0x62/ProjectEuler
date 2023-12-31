/// Each term in the Fibonacci sequence is generated by addind the
/// 2 previous terms
/// 1, 2, 3, 5, 8, 13, 21, ...
///
/// Find the sum of all EVEN values who do not exceed 4 million

pub fn solve() -> u32 {
    let max: u32 = 4_000_000;
    // let max = 100;
    let mut sum: u32 = 0;

    let mut a = 0;
    let mut b = 1;
    let mut fib = 1;

    while fib < max {
        fib = a + b;
        a = b;
        b = fib;

        if fib % 2 == 0 {
            sum += fib;
        }
    }

    sum
}
