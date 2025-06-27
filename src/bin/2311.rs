use std::{
    char,
    collections::{HashMap, HashSet, VecDeque},
    ops::Index,
};

/// Category: algorithms
/// Level: Medium
/// Percent: 38.127216%

/// You are given a binary string s and a positive integer k.
///
/// Return the length of the longest subsequence of s that makes up a binary number less than or equal to k.
///
/// Note:
///
///
/// 	The subsequence can contain leading zeroes.
/// 	The empty string is considered to be equal to 0.
/// 	A subsequence is a string that can be derived from another string by deleting some or no characters without changing the order of the remaining characters.
///
///
///
/// Example 1:
///
/// Input: s = "1001010", k = 5
/// Output: 5
/// Explanation: The longest subsequence of s that makes up a binary number less than or equal to 5 is "00010", as this number is equal to 2 in decimal.
/// Note that "00100" and "00101" are also possible, which are equal to 4 and 5 in decimal, respectively.
/// The length of this subsequence is 5, so 5 is returned.
///
///
/// Example 2:
///
/// Input: s = "00101001", k = 1
/// Output: 6
/// Explanation: "000001" is the longest subsequence of s that makes up a binary number less than or equal to 1, as this number is equal to 1 in decimal.
/// The length of this subsequence is 6, so 6 is returned.
///
///
///
/// Constraints:
///
///
/// 	1 <= s.length <= 1000
/// 	s[i] is either '0' or '1'.
/// 	1 <= k <= 10⁹
///

impl Solution {
    pub fn longest_subsequence(s: String, k: i32) -> i32 {
        // We can solve this problem using greedy
        // Let's iterate from right to left, take all the zeroes, and only take 1 if it doesn't make sum exceed k
        let mut res = 0i32;
        let mut sum = 0;
        for c in s.chars().rev() {
            if c == '0' {
                res += 1;
            } else {
                // Only take it if it doesn't bump k
                // Although k is designed to fit in i32, 2^res is not. So we must check if res is still under log2k
                if res <= k.ilog2() as i32 {
                    let bump = 2i32.pow(res as u32);
                    if sum + bump <= k {
                        sum += bump;
                        res += 1;
                    }
                }
            }
        }
        res
    }
}
