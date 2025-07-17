/// Category: algorithms
/// Level: Medium
/// Percent: 37.8391%

/// You are given an integer array nums.
/// A subsequence sub of nums with length x is called valid if it satisfies:
///
///
/// 	(sub[0] + sub[1]) % 2 == (sub[1] + sub[2]) % 2 == ... == (sub[x - 2] + sub[x - 1]) % 2.
///
///
/// Return the length of the longest valid subsequence of nums.
///
/// A subsequence is an array that can be derived from another array by deleting some or no elements without changing the order of the remaining elements.
///
///
/// Example 1:
///
///
/// Input: nums = [1,2,3,4]
///
/// Output: 4
///
/// Explanation:
///
/// The longest valid subsequence is [1, 2, 3, 4].
///
///
/// Example 2:
///
///
/// Input: nums = [1,2,1,1,2,1,2]
///
/// Output: 6
///
/// Explanation:
///
/// The longest valid subsequence is [1, 2, 1, 2, 1, 2].
///
///
/// Example 3:
///
///
/// Input: nums = [1,3]
///
/// Output: 2
///
/// Explanation:
///
/// The longest valid subsequence is [1, 3].
///
///
///
/// Constraints:
///
///
/// 	2 <= nums.length <= 2 * 10⁵
/// 	1 <= nums[i] <= 10⁷
///

impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        // We check 3 cases with different parities
        let case1 = count_len(&nums, 0);
        let case2 = count_len(&nums, 1);
        let first_different_parity = nums.iter().position(|x| (x + nums[0]) % 2 == 1);
        let mut case3 = 0;
        if let Some(first_different_parity) = first_different_parity {
            case3 = count_len(&nums[first_different_parity..], 0);
        }
        case1.max(case2).max(case3)
    }
}

fn count_len(nums: &[i32], parity: i32) -> i32 {
    let mut res = 1;
    let mut base = nums[0];
    for num in nums[1..].iter() {
        if (num + base) % 2 == parity {
            res += 1;
            base = *num;
        }
    }
    res
}
