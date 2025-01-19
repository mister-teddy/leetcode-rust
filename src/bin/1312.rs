/// 1312. Minimum Insertion Steps to Make a String Palindrome
/// Given a string s. In one step you can insert any character at any index of the string.
/// Return the minimum number of steps to make s palindrome.
/// A Palindrome String is one that reads the same backward as well as forward.
///
/// Example 1:
/// Input: s = "zzazz"
/// Output: 0
/// Explanation: The string "zzazz" is already palindrome we do not need any insertions.
///
/// Example 2:
/// Input: s = "mbadm"
/// Output: 2
/// Explanation: String can be "mbdadbm" or "mdbabdm".
///
/// Example 3:
/// Input: s = "leetcode"
/// Output: 5
/// Explanation: Inserting 5 characters the string becomes "leetcodocteel".
///
/// Constraints:
/// 1 <= s.length <= 500
/// s consists of lowercase English letters.
struct Solution {}

fn main() {
    assert_eq!(Solution::min_insertions("zzazz".to_string()), 0);
    assert_eq!(Solution::min_insertions("mbadm".to_string()), 2);
    assert_eq!(Solution::min_insertions("leetcode".to_string()), 5);
    assert_eq!(Solution::min_insertions("a".to_string()), 0);
    assert_eq!(Solution::min_insertions("ab".to_string()), 1);
    assert_eq!(Solution::min_insertions("racecar".to_string()), 0);
    assert_eq!(Solution::min_insertions("abcde".to_string()), 4);
}

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        // We can solve this problem using dynamic programming
        // Let dp[i][j] be minimum number of insertions to make s[i..j] palindrome
        let bytes = s.into_bytes();
        // let mut dp = vec![vec![0; chars.len()]; chars.len()];

        // Base case: dp[i][j] = 0 when j <= i
        // Recursive case: dp[i][j] = if s[i] == s[j] { 0 + dp[i+1][j-1] } else { 1  + min(dp[i+1][j], dp[i][j-1]) }
        // Optimisation: Because we only need dp[i+1] to calculate dp[i], we can use a 1D array instead of a 2D matrix
        let mut dp = vec![0; bytes.len()];

        for i in (0..bytes.len()).rev() {
            let mut prev = 0;
            for j in i + 1..bytes.len() {
                let val = if bytes[i] == bytes[j] {
                    // dp[i+1][j-1] is equivalent to "the previous iteration > the previous calculation", which already overwritten, so we need a tmp value here
                    prev
                } else {
                    // 1  + min(dp[i+1][j], dp[i][j-1])
                    1 + dp[j].min(dp[j - 1])
                };
                prev = dp[j];
                dp[j] = val;
            }
        }

        *dp.last().unwrap()
    }
}
