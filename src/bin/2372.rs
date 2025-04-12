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

use std::collections::HashSet;

// 1 <= n <= 10
// 1 <= k <= 9
struct Solution {}

impl Solution {
    pub fn count_good_integers(n: i32, k: i32) -> i64 {
        let nums = find_all_k_palindrome(n as usize, vec![], 0);
        println!("{:?}", nums);

        0
    }
}

/// This function returns all k-palindrome with length n
/// We find then by recursive/backtracking
/// In each recursion cycle, we need to pass in all posible prefixes
/// This function assumes all prefixes have the same length
fn find_all_k_palindrome(n: usize, prefixes: Vec<u8>, used_mask: i32) -> HashSet<u64> {
    // This is the backtracking tail, concat the digits so that we have a valid number
    let i = prefixes.len();
    if i == n {
        let num = prefixes.iter().fold(0 as u64, |s, d| 10 * s + *d as u64);
        return [num].into();
    }

    // i is smaller than n
    // Handle 2 cases here: left half and right half
    let mut res = HashSet::new();
    if i < (n + 1) / 2 {
        // left half
        for d in if i == 0 { 1 } else { 0 }..=9 {
            // try each *digit* from 0 to 9
            let mut prefixes = prefixes.clone();
            prefixes.push(d);
            res.extend(find_all_k_palindrome(n, prefixes, used_mask));
        }
    } else {
        // right half
        for di in 0..n / 2 {
            // try each *index* in the left half of prefixes
            if (used_mask >> di) & 1 == 0 {
                // di not used before
                let x = prefixes[di].clone();
                let mut prefixes = prefixes.clone();
                prefixes.push(x);
                res.extend(find_all_k_palindrome(n, prefixes, used_mask | (1 << di)));
            }
        }
    }

    res
}

fn main() {
    // Example 1
    assert_eq!(Solution::count_good_integers(3, 5), 27);

    // Example 2
    assert_eq!(Solution::count_good_integers(1, 4), 2);

    // Example 3
    assert_eq!(Solution::count_good_integers(5, 6), 2468);
}
