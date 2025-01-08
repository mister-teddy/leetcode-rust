/// 673. Number of Longest Increasing Subsequence
///
/// Given an integer array nums, return the number of longest increasing subsequences.
/// Notice that the sequence has to be strictly increasing.
///
/// Example 1:
/// Input: nums = [1,3,5,4,7]
/// Output: 2
/// Explanation: The two longest increasing subsequences are [1, 3, 4, 7] and [1, 3, 5, 7].
///
/// Example 2:
/// Input: nums = [2,2,2,2,2]
/// Output: 5
/// Explanation: The length of the longest increasing subsequence is 1, and there are 5 increasing subsequences of length 1, so output 5.
///
/// Constraints:
/// 1 <= nums.length <= 2000
/// -106 <= nums[i] <= 106
/// The answer is guaranteed to fit inside a 32-bit integer.
struct Solution;

fn main() {
    assert_eq!(Solution::find_number_of_lis(vec![1, 3, 5, 4, 7]), 2);
    assert_eq!(Solution::find_number_of_lis(vec![2, 2, 2, 2, 2]), 5);
    assert_eq!(
        Solution::find_number_of_lis(vec![1, 2, 4, 3, 5, 4, 7, 2]),
        3
    );
    assert_eq!(
        Solution::find_number_of_lis(vec![1, 2, 4, 3, 5, 4, 7, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
        4
    );
}

impl Solution {
    /// Let dp[i] be the length of the longest increasing subsequence ending at index i.
    ///
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len() + 1];
        let mut counts = vec![1; nums.len() + 1];
        for i in 1..=nums.len() {
            let mut count = 1;
            let mut longest = 1;
            for j in 1..i {
                if nums[j - 1] < nums[i - 1] {
                    if dp[j] + 1 > longest {
                        longest = dp[j] + 1;
                        count = counts[j];
                    } else if dp[j] + 1 == longest {
                        count += counts[j];
                    }
                }
            }
            dp[i] = longest;
            counts[i] = count;
        }
        let max = *dp.iter().max().unwrap();
        return counts
            .iter()
            .enumerate()
            .filter(|(i, _)| dp[*i] == max)
            .map(|(_, val)| val)
            .sum();
    }
}
