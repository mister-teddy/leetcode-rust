/// # 115. Distinct Subsequences
/// Given two strings s and t, return the number of distinct subsequences of s which equals t.
///
/// The test cases are generated so that the answer fits on a 32-bit signed integer.
///
/// Example 1:
/// Input: s = "rabbbit", t = "rabbit"
/// Output: 3
/// Explanation:
/// As shown below, there are 3 ways you can generate "rabbit" from s.
/// rabbbit
/// rabbbit
/// rabbbit
///
/// Example 2:
/// Input: s = "babgbag", t = "bag"
/// Output: 5
/// Explanation:
/// As shown below, there are 5 ways you can generate "bag" from s.
/// babgbag
/// babgbag
/// babgbag
/// babgbag
/// babgbag
///
/// Constraints:
/// 1 <= s.length, t.length <= 1000
/// s and t consist of English letters.
struct Solution;

fn main() {
    assert_eq!(
        Solution::num_distinct("rabbbit".to_string(), "rabbit".to_string()),
        3
    );
    assert_eq!(
        Solution::num_distinct("babgbag".to_string(), "bag".to_string()),
        5
    );
}

impl Solution {
    /// We denote dp[i][j] as the number of distinct subsequences of s[..i] which equals t[..j]
    /// It is clear that:
    /// - dp[0][0] = 1 because there is only 1 way to build "" from ""
    /// - dp[i][0] = 0 because it is agains the contrains
    /// - dp[0][j] = 0 because you cannot have any subsequences of an empty string "" that equals to another non empty string (although also against the constrains)
    /// - dp[i][j] = dp[i-1][j] + (if (s[i-1] == t[j-1]) { dp[i-1][j-1]) } else { 0 }) because you can either:
    ///   + form t[..j] from the previous characters without using the current character
    ///   + form t[..j-1] from the previous characters only if you can use the current character s[i-1] to build the last character of the current result (t[j-1])
    pub fn num_distinct(s: String, t: String) -> i32 {
        let mut dp = vec![0; t.len() + 1];
        dp[0] = 1;
        let bytes_s = s.as_bytes();
        let bytes_t = t.as_bytes();
        for i in 1..=s.len() {
            let mut tmp = dp.clone();
            for j in 1..=t.len() {
                tmp[j] = dp[j]
                    + (if bytes_s[i - 1] == bytes_t[j - 1] {
                        dp[j - 1]
                    } else {
                        0
                    });
            }
            dp = tmp;
        }
        dp[t.len()].try_into().unwrap()
    }
}
