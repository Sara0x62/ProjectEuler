/// Problem 5
/// Smallest multiple
/// 2520 is the smallest number that can be divided by the numbers 0 to 10 with 0 remainder
/// What is the smallest positive number dividable from 0 to 20 with 0 remainder
pub fn solve() -> u64 {
    let mut flag: bool = false;
    let mut out: u64 = 0;

    // Simple bruteforce solution - it slow tho
    for n in 1..u64::MAX {
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

use std::collections::HashMap;

/// Solve2 uses the method;
/// Finding the "LCM" (lowest common multiple)
/// Using the "Prime factors method"
use crate::util;
pub fn solve2(max: u64) -> u64 {
    let mut out: u64 = 1;

    // Get prime factors
    let factors = prime_factors(max);

    // Factors are stored in a hashmap
    // Key 'k' is the number,
    // Value 'v' is the amount of times we multiply it
    // So we use K to the power of V (k^v)
    for (k, v) in factors.iter() {
        out *= k.pow(*v as u32);
    }

    out
}

pub fn prime_factors(n: u64) -> HashMap<u64, u64> {
    let mut v: HashMap<u64, u64> = HashMap::new();
    let mut map: HashMap<u64, u64> = HashMap::with_capacity(n as usize);

    let mut tmp: u64;
    let mut prime: u64;
    for i in 2..=n {
        tmp = i;    // Temporary, so we can edit the variable easier
        prime = 2;  // Reset to first prime number

        while tmp > 1 {
            if tmp % prime == 0 {
                // prime is a factor for the number
                *v.entry(prime).or_insert(0) += 1; // Add to our list of primes
                tmp /= prime; // Divide
            } else {
                // Normal unwrap should be fine..
                // Simply gets the next possible prime number
                prime = util::next_prime(prime, &tmp).unwrap();
            }
        }

        for (k, v) in v.iter() {
            // See if entry exists,
            // Modify if new value is bigger
            // Insert if it does not exist
            // We dont need the return value
            _ = *map.entry(*k).and_modify(|f|
                if *f < *v { *f = *v; }
            ).or_insert(*v);
        }

        v.clear();  // Reset hashmap
    }

    map
}
