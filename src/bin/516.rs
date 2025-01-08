/// 516. Longest Palindromic Subsequence
/// Given a string s, find the longest palindromic subsequence's length in s.
/// A subsequence is a sequence that can be derived from another sequence by deleting some or no elements without changing the order of the remaining elements.

/// Example 1:
/// Input: s = "bbbab"
/// Output: 4
/// Explanation: One possible longest palindromic subsequence is "bbbb".

/// Example 2:
/// Input: s = "cbbd"
/// Output: 2
/// Explanation: One possible longest palindromic subsequence is "bb".

/// Constraints:
/// 1 <= s.length <= 1000
/// s consists only of lowercase English letters.
use std::vec;

fn longest_palindrome_subseq(s: String) -> i32 {
    let bytes = s.as_bytes();
    let mut dp = vec![0; s.len()];

    for i in 0..s.len() {
        let mut tmp = vec![0; s.len()];
        tmp[i] = 1;
        for j in (0..i).rev() {
            tmp[j] = core::cmp::max(dp[j], tmp[j + 1]);
            tmp[j] = core::cmp::max(tmp[j], dp[j + 1] + if bytes[i] == bytes[j] { 2 } else { 0 })
        }
        dp = tmp;
    }

    dp[0]
}

fn main() {
    assert_eq!(longest_palindrome_subseq("y".to_string()), 1);
    assert_eq!(longest_palindrome_subseq("bbbab".to_string()), 4);
    assert_eq!(longest_palindrome_subseq("cbbd".to_string()), 2);
}
