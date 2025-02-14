/// # 474. Ones and Zeroes
///
/// You are given an array of binary strings strs and two integers m and n.
///
/// Return the size of the largest subset of strs such that there are at most m 0's and n 1's in the subset.
///
/// A set x is a subset of a set y if all elements of x are also elements of y.
///
/// Example 1:
///
/// Input: strs = ["10","0001","111001","1","0"], m = 5, n = 3
/// Output: 4
/// Explanation: The largest subset with at most 5 0's and 3 1's is {"10", "0001", "1", "0"}, so the answer is 4.
/// Other valid but smaller subsets include {"0001", "1"} and {"10", "1", "0"}.
/// {"111001"} is an invalid subset because it contains 4 1's, greater than the maximum of 3.
///
/// Example 2:
///
/// Input: strs = ["10","0","1"], m = 1, n = 1
/// Output: 2
/// Explanation: The largest subset is {"0", "1"}, so the answer is 2.
///
/// Constraints:
///
/// 1 <= strs.length <= 600
/// 1 <= strs[i].length <= 100
/// strs[i] consists only of digits '0' and '1'.
/// 1 <= m, n <= 100
struct Solution {}

impl Solution {
    /// Extract the numbers of 0 and 1 bits in each string
    fn extract(s: &String) -> (usize, usize) {
        let mut zeroes = 0;
        let mut ones = 0;
        for &byte in s.as_bytes() {
            match byte {
                b'0' => zeroes += 1,
                b'1' => ones += 1,
                _ => (),
            }
        }
        (zeroes, ones)
    }

    // pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
    //     // First, let's solve this problem using backtracking
    //     let mut res = 0;
    //     // Look through all possibilities and pick the best result
    //     for (i, str) in strs.iter().enumerate() {
    //         let (zeroes, ones) = Solution::extract(str);
    //         if m >= zeroes && n >= ones {
    //             let mut new_strs = strs.clone();
    //             new_strs.remove(i);
    //             // Recursive call
    //             res = res.max(Solution::find_max_form(new_strs, m - zeroes, n - ones) + 1);
    //         }
    //     }
    //     res
    // }

    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let nums = strs.iter().map(Solution::extract);
        // From the top down backtracking approach above, we can implement a dynamic programming approach
        // using bottom up with the same logic.
        // Because each recursive call require 3 parameters, we'll store the smaller results into a 3D array
        // R1: Each iteration over a str overwrites the previous dp array, so we only need a 2D array
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for (zeroes, ones) in nums {
            // Reversion is necessary here because we need to calculate dp[k][i][j] from dp[k-1][<i][<j].
            // Otherwise, dp[k-1] data would be overwritten
            for i in (zeroes..=m).rev() {
                for j in (ones..=n).rev() {
                    // Check if we can take the current str
                    // R2: Instead of looping from 0 and do the checking, we can achieve both with better
                    // performance by looping from zeroes and ones
                    // If we do take it, consume it as well as the equivalent numbers of 0 and 1
                    dp[i][j] = dp[i][j].max(dp[i - zeroes][j - ones] + 1);
                }
            }
        }
        dp[m][n]
    }
}

fn main() {
    assert_eq!(
        Solution::find_max_form(
            vec![
                "10".to_string(),
                "0001".to_string(),
                "111001".to_string(),
                "1".to_string(),
                "0".to_string()
            ],
            5,
            3
        ),
        4
    );
    assert_eq!(
        Solution::find_max_form(
            vec!["10".to_string(), "0".to_string(), "1".to_string()],
            1,
            1
        ),
        2
    );
}
