/// 416. Partition Equal Subset Sum
/// Given an integer array nums, return true if you can partition the array into two subsets such that the sum of the elements in both subsets is equal or false otherwise.
/// Example 1:
/// Input: nums = [1,5,11,5]
/// Output: true
/// Explanation: The array can be partitioned as [1, 5, 5] and [11].
/// Example 2:
/// Input: nums = [1,2,3,5]
/// Output: false
/// Explanation: The array cannot be partitioned into equal sum subsets.
/// Constraints:
/// 1 <= nums.length <= 200
/// 1 <= nums[i] <= 100
struct Solution {}

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        // First, we calculate the total sum S.
        let sum: usize = nums.iter().map(|&x| x as usize).sum();

        // Note: We don't need to check the entire sum S, just half of it. Also, if S is odd, no subset can sum to S/2.
        if sum % 2 != 0 {
            return false;
        }
        let target = sum / 2;

        // Then, we implement dynamic programming to check for a subset with the target sum S/2.
        let mut dp = vec![false; target + 1];
        dp[0] = true;

        for &num in &nums {
            let num = num as usize;
            for j in (num..=target).rev() {
                dp[j] |= dp[j - num]; // If a subset with sum = target - num exists, then a subset with sum = target also exists.
            }
        }

        dp[target] // The result of this problem is the ability to form a subset with the target sum (S/2).
    }
}

fn main() {
    assert_eq!(Solution::can_partition(vec![1, 5, 11, 5]), true);
    assert_eq!(Solution::can_partition(vec![1, 2, 3, 5]), false);
}
