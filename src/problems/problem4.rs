/// Problem 4
/// Largest Palindrome Product
/// Find the largest palindrome product of 3 digits
/// example for 2 digits - 9009 = 91 x 99.
pub fn solve(a: u32, b: u32) -> u32 {
    let mut max: u32 = 0;
    for x in (0..=a).rev() {
        for y in (0..=b).rev() {
            if is_palindrome(x * y) && max < x * y {
                max = x * y;
            }
        }
    }
    max
}

pub fn is_palindrome(d: u32) -> bool {
    let mut d2 = d;
    let mut rev = 0;
    let mut dgt: u32;

    while d2 != 0 {
        // while numbers are left..
        dgt = d2 % 10; // Pop last digit
        rev = rev * 10 + dgt; // Add last digit to the reversed number
        d2 /= 10; // divide by 10, getting rid of the digit we just added to rev
    }

    if rev == d {
        return true;
    }

    false
}
