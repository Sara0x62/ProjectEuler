mod problems;
use problems::*;
fn main() {
    println!("Solve for 1 - {}", problem1::solve(1000));
    println!("Solve for 2 - {}", problem2::solve());
    println!("Solve for 3 - {}", problem3::solve(600_851_475_143));
    println!("Solve for 4 - {}", problem4::solve(999, 999));
    println!("Solve for 5 - {}", problem5::solve2(20));
    println!("Solve for 6 - {}", problem6::solve(100));
}
