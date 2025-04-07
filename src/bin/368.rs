// 368. Largest Divisible Subset
// Medium
// Topics
// Companies
// Given a set of distinct positive integers nums, return the largest subset answer such that every pair (answer[i], answer[j]) of elements in this subset satisfies:

// answer[i] % answer[j] == 0, or
// answer[j] % answer[i] == 0
// If there are multiple solutions, return any of them.

// Example 1:

// Input: nums = [1,2,3]
// Output: [1,2]
// Explanation: [1,3] is also accepted.
// Example 2:

// Input: nums = [1,2,4,8]
// Output: [1,2,4,8]

// Constraints:

// 1 <= nums.length <= 1000
// 1 <= nums[i] <= 2 * 109
// All the integers in nums are unique.
struct Solution {}

impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        let mut sorted = nums.clone();
        sorted.sort();

        let mut dp = vec![1; nums.len()];
        let mut prevs = vec![nums.len(); nums.len()];
        let mut max_i = 0;

        for (i, num) in sorted.iter().enumerate() {
            for (j, prev) in sorted[0..i].iter().enumerate() {
                if num % prev == 0 {
                    if dp[j] + 1 > dp[i] {
                        dp[i] = dp[j] + 1;
                        prevs[i] = j;
                    }
                }
            }
            if (dp[i] > dp[max_i]) {
                max_i = i;
            }
        }

        let mut res = vec![];
        while max_i != nums.len() {
            res.insert(0, sorted[max_i]);
            max_i = prevs[max_i];
        }

        res
    }
}
