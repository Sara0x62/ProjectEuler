pub mod problem1;
pub mod problem2;
pub mod problem3;
pub mod problem4;
pub mod problem5;
pub mod problem6;
pub mod util;

#[cfg(test)]
mod tests {
    use crate::problems::problem1;
    use crate::problems::problem3;
    use crate::problems::problem4;
    use crate::problems::problem5;
    use crate::problems::problem6;
    use crate::util;

    #[test]
    fn problem1() {
        // Given test case,
        // the sum of all multiples of 3 or 5 below 10
        // = 23
        // for 3 + 5 + 6 + 9 -> 23 | not including 10
        let a = 23;
        let test = problem1::solve(10);
        assert_eq!(a, test);
    }

    // Problem 2 - No test case given
    // ...

    #[test]
    fn problem3() {
        // Given test case,
        // the largest prime factor of 13195 = 29
        // (out of factors: 5, 7, 13 and 29)
        let i = 13195;
        let a = 29;

        let res = problem3::solve(i);
        assert_eq!(a, res);
    }

    #[test]
    fn prime_test() {
        // 29 is a prime number, 2639 is not (dividable by 7)
        assert_eq!(true, util::is_prime(29));
        assert_eq!(false, util::is_prime(2639));
    }

    #[test]
    fn palindrome_test() {
        assert_eq!(true, problem4::is_palindrome(9009));
        assert_eq!(false, problem4::is_palindrome(12));
    }

    #[test]
    fn problem5_solve2() {
        // let solve1 = problem5::solve();
        let solve2 = problem5::solve2(10);

        // assert_eq!(solve1, solve2);
        assert_eq!(2520, solve2);
    }

    #[test]
    fn problem6() {
        assert_eq!(2640, problem6::solve(10));
    }
}
