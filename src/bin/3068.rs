/// Category: algorithms
/// Level: Hard
/// Percent: 65.21237%

/// There exists an undirected tree with n nodes numbered 0 to n - 1. You are given a 0-indexed 2D integer array edges of length n - 1, where edges[i] = [ui, vi] indicates that there is an edge between nodes ui and vi in the tree. You are also given a positive integer k, and a 0-indexed array of non-negative integers nums of length n, where nums[i] represents the value of the node numbered i.
///
/// Alice wants the sum of values of tree nodes to be maximum, for which Alice can perform the following operation any number of times (including zero) on the tree:
///
///
/// 	Choose any edge [u, v] connecting the nodes u and v, and update their values as follows:
///
///
/// 		nums[u] = nums[u] XOR k
/// 		nums[v] = nums[v] XOR k
///
///
///
///
/// Return the maximum possible sum of the values Alice can achieve by performing the operation any number of times.
///
///
/// Example 1:
///
/// Input: nums = [1,2,1], k = 3, edges = [[0,1],[0,2]]
/// Output: 6
/// Explanation: Alice can achieve the maximum sum of 6 using a single operation:
/// - Choose the edge [0,2]. nums[0] and nums[2] become: 1 XOR 3 = 2, and the array nums becomes: [1,2,1] -> [2,2,2].
/// The total sum of values is 2 + 2 + 2 = 6.
/// It can be shown that 6 is the maximum achievable sum of values.
///
///
/// Example 2:
///
/// Input: nums = [2,3], k = 7, edges = [[0,1]]
/// Output: 9
/// Explanation: Alice can achieve the maximum sum of 9 using a single operation:
/// - Choose the edge [0,1]. nums[0] becomes: 2 XOR 7 = 5 and nums[1] become: 3 XOR 7 = 4, and the array nums becomes: [2,3] -> [5,4].
/// The total sum of values is 5 + 4 = 9.
/// It can be shown that 9 is the maximum achievable sum of values.
///
///
/// Example 3:
///
/// Input: nums = [7,7,7,7,7,7], k = 3, edges = [[0,1],[0,2],[0,3],[0,4],[0,5]]
/// Output: 42
/// Explanation: The maximum achievable sum is 42 which can be achieved by Alice performing no operations.
///
///
///
/// Constraints:
///
///
/// 	2 <= n == nums.length <= 2 * 10⁴
/// 	1 <= k <= 10⁹
/// 	0 <= nums[i] <= 10⁹
/// 	edges.length == n - 1
/// 	edges[i].length == 2
/// 	0 <= edges[i][0], edges[i][1] <= n - 1
/// 	The input is generated such that edges represent a valid tree.
///

impl Solution {
    pub fn maximum_value_sum(nums: Vec<i32>, k: i32, edges: Vec<Vec<i32>>) -> i64 {
        // The IMPORTANT TRICK is this: we can toggle ANY two nodes, regardless of adjacency, due to the nature of the XOR operator and the given connected tree
        // So this problem becomes: "Replace AN EVEN NUMBER of `num` with `num XOR k` so that the sum is maximal."

        // We can solve this problem with dynamic programming
        // Let dp[i][c] be the maximum possible sum of the tree with:
        // - Only the first `i` nodes considered
        // - And dp[i].0 is when we flipped an even number of nodes
        // -     dp[i].1 is when we flipped an odd  number of nodes
        // Base cases: no node traversed, no sum. Also, you cannot flip an odd number of nodes from an empty node set
        let mut dp = (0, i64::MIN);
        // Recursive cases: at each node, we can either flip it or not:
        // - If we leave it untouched, then:    dp[i][c] = dp[i-1][c]  + num
        // - If we decide to flip it, then:     dp[i][c] = dp[i-1][!c] + num XOR k
        // Take the max of these two
        // It is obvious that we only need the previously calculated value (dp[i-1] is enough to calculate dp[i]), so we can use a single dp value instead of a dp[i] array
        for num in nums {
            dp = (
                (dp.0 + num as i64).max(dp.1 + (num ^ k) as i64),
                (dp.1 + num as i64).max(dp.0 + (num ^ k) as i64),
            );
        }
        dp.0
    }
}
