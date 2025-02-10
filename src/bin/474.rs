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
    fn extract(s: &String) -> (i32, i32) {
        let mut zeroes = 0;
        let mut ones = 0;
        for c in s.chars() {
            if c == '0' {
                zeroes += 1;
            } else {
                ones += 1;
            }
        }
        return (zeroes, ones);
    }

    // pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
    //     let m = m as usize;
    //     let n = n as usize;
    //     let nums = strs.iter().map(Solution::extract);
    //     // We can solve this problem using dynamic programming
    //     // The pattern is similar to coin exchange
    //     let mut dp = vec![vec![0; n + 1]; m + 1];
    //     for i in 1..=m {
    //         for j in 1..=n {
    //             for (zeroes, ones) in nums.clone() {
    //                 dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
    //                 // If we take this binary string, we consume the exact amounts of 0 and 1 bits
    //                 if i >= zeroes && j >= ones {
    //                     dp[i][j] = dp[i][j].max(dp[i - zeroes][j - ones] + 1);
    //                 }
    //             }
    //         }
    //         println!("{:?}", dp);
    //     }
    //     dp[m][n]
    // }

    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        // First, let's solve this problem using backtracking
        let mut res = 0;
        // Look through all possibilities and pick the best result
        for (i, str) in strs.iter().enumerate() {
            let (zeroes, ones) = Solution::extract(str);
            if m >= zeroes && n >= ones {
                let mut new_strs = strs.clone();
                new_strs.remove(i);
                // Recursive call
                res = res.max(Solution::find_max_form(new_strs, m - zeroes, n - ones) + 1);
            }
        }
        res
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
