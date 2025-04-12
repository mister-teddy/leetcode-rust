// 3272. Find the Count of Good Integers
// Hint
// You are given two positive integers n and k.

// An integer x is called k-palindromic if:

// x is a palindrome.
// x is divisible by k.
// An integer is called good if its digits can be rearranged to form a k-palindromic integer. For example, for k = 2, 2020 can be rearranged to form the k-palindromic integer 2002, whereas 1010 cannot be rearranged to form a k-palindromic integer.

// Return the count of good integers containing n digits.

// Note that any integer must not have leading zeros, neither before nor after rearrangement. For example, 1010 cannot be rearranged to form 101.

// Example 1:

// Input: n = 3, k = 5

// Output: 27

// Explanation:

// Some of the good integers are:

// 551 because it can be rearranged to form 515.
// 525 because it is already k-palindromic.
// Example 2:

// Input: n = 1, k = 4

// Output: 2

// Explanation:

// The two good integers are 4 and 8.

// Example 3:

// Input: n = 5, k = 6

// Output: 2468

// Constraints:
// 1 <= n <= 10
// 1 <= k <= 9
struct Solution {}

use std::{collections::HashSet, env::consts::FAMILY, f32::DIGITS};

impl Solution {
    pub fn count_good_integers(n: i32, k: i32) -> i64 {
        let mut nums = find_all_k_palindrome(n as usize, k, vec![]);
        nums.iter().map(count_combinations).sum()
    }
}

/// This function returns all k-palindrome with length n
/// We find then by recursive/backtracking
/// In each recursion cycle, we need to pass in all posible prefixes
/// This function assumes all prefixes have the same length
fn find_all_k_palindrome(n: usize, k: i32, prefixes: Vec<u8>) -> HashSet<Vec<u8>> {
    // This is the backtracking tail, concat the digits so that we have a valid number
    let i = prefixes.len();
    if i == n {
        let num = prefixes.iter().fold(0 as u64, |s, d| 10 * s + *d as u64);
        if num % k as u64 == 0 {
            let mut prefixes = prefixes.clone();
            prefixes.sort();
            return [prefixes].into();
        } else {
            return [].into();
        }
    }

    // Handle 2 cases here: left half and right half
    let mut res = HashSet::new();
    if i < (n + 1) / 2 {
        // left half
        for d in if i == 0 { 1 } else { 0 }..=9 {
            // try each *digit* from 0 to 9
            let mut prefixes = prefixes.clone();
            prefixes.push(d);
            res.extend(find_all_k_palindrome(n, k, prefixes));
        }
    } else {
        // right half
        // just pick the mirrored digit
        let x = prefixes[n - i - 1].clone();
        let mut prefixes = prefixes.clone();
        prefixes.push(x);
        res.extend(find_all_k_palindrome(n, k, prefixes));
    }

    res
}

fn factorial(x: usize) -> i64 {
    if x == 0 {
        return 1;
    }
    x as i64 * factorial(x - 1)
}

fn count_combinations(digits: &Vec<u8>) -> i64 {
    let mut all = if digits[0] == 0 {
        digits.iter().filter(|x| **x != 0).count() as i64 * factorial(digits.len() - 1)
    } else {
        factorial(digits.len())
    };
    let mut count = 1;
    for i in 1..digits.len() {
        if digits[i] == digits[i - 1] {
            count += 1;
        } else {
            all /= factorial(count);
            count = 1;
        }
    }
    all /= factorial(count);
    all
}

fn main() {
    // Example 1
    assert_eq!(Solution::count_good_integers(3, 5), 27);

    // Example 2
    assert_eq!(Solution::count_good_integers(1, 4), 2);

    // Example 3
    assert_eq!(Solution::count_good_integers(5, 6), 2468);
}
