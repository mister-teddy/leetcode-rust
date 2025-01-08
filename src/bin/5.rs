/// 5. Longest Palindromic Substring
/// Given a string s, return the longest palindromic substring in s.

/// Example 1:
/// Input: s = "babad"
/// Output: "bab"
/// Explanation: "aba" is also a valid answer.

/// Example 2:
/// Input: s = "cbbd"
/// Output: "bb"

/// Constraints:
/// 1 <= s.length <= 1000
/// s consist of only digits and English letters.
use std::vec;

fn longest_palindrome_substring(s: String) -> String {
    let bytes = s.as_bytes();
    let mut dp = vec![false; s.len()];
    let mut res = &s[0..1];

    for i in 1..s.len() {
        let mut tmp = vec![false; s.len()];
        dp[i] = true;
        tmp[i] = true;
        for j in 0..i {
            if bytes[i] == bytes[j] {
                tmp[j] = dp[j + 1];
            }
            if tmp[j] && res.len() < i - j + 1 {
                res = &s[j..=i];
            }
        }
        dp = tmp;
    }

    res.to_string()
}

fn main() {
    assert_eq!(longest_palindrome_substring("ababad".to_string()), "ababa");
    assert_eq!(longest_palindrome_substring("cbbd".to_string()), "bb");
}
