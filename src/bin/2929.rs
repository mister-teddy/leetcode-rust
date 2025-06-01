use std::panic::AssertUnwindSafe;

/// Category: algorithms
/// Level: Medium
/// Percent: 34.860428%

/// You are given two positive integers n and limit.
///
/// Return the total number of ways to distribute n candies among 3 children such that no child gets more than limit candies.
///
///
/// Example 1:
///
/// Input: n = 5, limit = 2
/// Output: 3
/// Explanation: There are 3 ways to distribute 5 candies such that no child gets more than 2 candies: (1, 2, 2), (2, 1, 2) and (2, 2, 1).
///
///
/// Example 2:
///
/// Input: n = 3, limit = 3
/// Output: 10
/// Explanation: There are 10 ways to distribute 3 candies such that no child gets more than 3 candies: (0, 0, 3), (0, 1, 2), (0, 2, 1), (0, 3, 0), (1, 0, 2), (1, 1, 1), (1, 2, 0), (2, 0, 1), (2, 1, 0) and (3, 0, 0).
///
///
///
/// Constraints:
///
///
/// 	1 <= n <= 10⁶
/// 	1 <= limit <= 10⁶
///

// struct Solution {}

// fn main() {
//     assert_eq!(Solution::distribute_candies(5, 2), 3);
//     assert_eq!(Solution::distribute_candies(3, 3), 10);
//     assert_eq!(Solution::distribute_candies(1, 1), 3);
//     assert_eq!(Solution::distribute_candies(3, 1), 1);
//     assert_eq!(Solution::distribute_candies(1_000_000, 1_000_000), 1);
// }

impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i64 {
        // Let ways[i] be the number of ways to split i candies among 2 children with the given constrains.
        let mut ways = vec![0; n as usize + 1];
        for i in 0..=n {
            // To split i candies between 2 children, we can split 0..j (with j <= i, j <= limit) candies for the first one, and i - j (with i - j <= limit) candies for the second one.
            // => min(i, limit) >= j >= max(i - limit, 0)
            // So:
            ways[i as usize] = 0.max(i.min(limit) - 0.max(i - limit) + 1);
        }

        // let res be the number of ways to split n candies among 3 children with the given constrains.
        let mut res = 0i64;
        // In order to do so, we can split 0..j (with j <= n, j <= limit) candies for the first child, then n - j for the two other children,
        // for which the result had been computed previously in `ways`.
        for j in 0..=n.min(limit) as usize {
            res += ways[n as usize - j] as i64;
        }

        res
    }
}
