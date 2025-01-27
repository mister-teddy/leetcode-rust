/// 96. Unique Binary Search Trees
/// Medium
///
/// Given an integer n, return the number of structurally unique BST's (binary search trees) which has exactly n nodes of unique values from 1 to n.
///
/// Example 1:
///
/// Input: n = 3
/// Output: 5
///
/// Example 2:
///
/// Input: n = 1
/// Output: 1
///
/// Constraints:
///
/// 1 <= n <= 19
pub struct Solution;

fn main() {
    assert_eq!(Solution::num_trees(1), 1);
    assert_eq!(Solution::num_trees(2), 2);
    assert_eq!(Solution::num_trees(3), 5);
    assert_eq!(Solution::num_trees(4), 14);
    assert_eq!(Solution::num_trees(5), 42);
    assert_eq!(Solution::num_trees(6), 132);
    assert_eq!(Solution::num_trees(7), 429);
    assert_eq!(Solution::num_trees(8), 1430);
    assert_eq!(Solution::num_trees(9), 4862);
    assert_eq!(Solution::num_trees(10), 16796);
    assert_eq!(Solution::num_trees(11), 58786);
    assert_eq!(Solution::num_trees(12), 208012);
    assert_eq!(Solution::num_trees(13), 742900);
    assert_eq!(Solution::num_trees(14), 2674440);
    assert_eq!(Solution::num_trees(15), 9694845);
    assert_eq!(Solution::num_trees(16), 35357670);
    assert_eq!(Solution::num_trees(17), 129644790);
    assert_eq!(Solution::num_trees(18), 477638700);
    assert_eq!(Solution::num_trees(19), 1767263190);
}

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let n = n as usize;
        // We can solve this problem using dynamic programming
        // Let dp[start][end] be the number of ways to build a binary tree from
        // the numbers in the [start, end] range.
        let mut dp = vec![vec![1; n]; n];
        // The number of ways to build a binary tree from any range is to pick
        // a number as the root, then build the left subtree with the smaller
        // remaining numbers and the right subtree with the larger ones.
        // That is:
        // dp[start][end] = SUM(dp[start][i-1]*dp[i+1][end]), for all i between [start, end]
        for k in 1..n {
            for start in 0..n {
                let end = start + k;
                if end < n {
                    dp[start][end] = (start..=end).fold(0, |count, i| {
                        count
                            + (if i > start { dp[start][i - 1] } else { 1 })
                                * (if i < end { dp[i + 1][end] } else { 1 })
                    })
                }
            }
        }
        dp[0][n - 1]
    }
}
