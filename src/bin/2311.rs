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
        // I'm thinking about a dynamic programming approach
        // Where dp[i] = the longest subsequece ending with i
        // If you append 1 new binary digit, the prefix value will be doubled
        // So we need dpp[i] = the longest subsequence that <= k/2
        // First, we build an array of logs of k
        let mut ks = HashSet::new();
        fn build_ks(ks: &mut HashSet<i32>, k: i32) {
            ks.insert(k);
            if k > 0 {
                build_ks(ks, k / 2);
                if !ks.contains(&((k - 1) / 2)) {
                    build_ks(ks, (k - 1) / 2);
                }
            }
        }
        build_ks(&mut ks, k);
        // println!("ks = {:?}", ks);

        // Let dp[i][j] be the length longest subsequence that end with s[i], and the number is <= j
        // The result of the problem is max(dp[*][k])
        let mut res = 0;
        let chars: Vec<char> = s.chars().collect();
        let mut dp = vec![HashMap::new(); chars.len() + 1];
        // Obviously:
        for k in &ks {
            dp[0].insert(k, 0);
        }

        // dp[i][k] = dp[*][k/2]   // if s[i] == '0'
        // or = dp[*][(k-1)/2]     // if s[i] == '1'
        for i in 1..=chars.len() {
            let char = chars[i - 1];
            for k in &ks {
                dp[i].insert(k, 0);
                for prev in 0..i {
                    if k != &0 || char == '0' {
                        *dp[i].entry(k).or_insert(0) = dp[i][&k]
                            .max(dp[prev][&if char == '0' { k / 2 } else { (k - 1) / 2 }] + 1);
                    }
                }
            }
            res = res.max(dp[i][&k]);
        }
        // println!("dp = {:?}", dp);
        res
    }
}
