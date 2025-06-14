/// Category: algorithms
/// Level: Medium
/// Percent: 43.69886%

/// You are given a 0-indexed integer array nums and an integer p. Find p pairs of indices of nums such that the maximum difference amongst all the pairs is minimized. Also, ensure no index appears more than once amongst the p pairs.
///
/// Note that for a pair of elements at the index i and j, the difference of this pair is |nums[i] - nums[j]|, where |x| represents the absolute value of x.
///
/// Return the minimum maximum difference among all p pairs. We define the maximum of an empty set to be zero.
///
///
/// Example 1:
///
/// Input: nums = [10,1,2,7,1,3], p = 2
/// Output: 1
/// Explanation: The first pair is formed from the indices 1 and 4, and the second pair is formed from the indices 2 and 5.
/// The maximum difference is max(|nums[1] - nums[4]|, |nums[2] - nums[5]|) = max(0, 1) = 1. Therefore, we return 1.
///
///
/// Example 2:
///
/// Input: nums = [4,2,1,2], p = 1
/// Output: 0
/// Explanation: Let the indices 1 and 3 form a pair. The difference of that pair is |2 - 2| = 0, which is the minimum we can attain.
///
///
///
/// Constraints:
///
///
/// 	1 <= nums.length <= 10⁵
/// 	0 <= nums[i] <= 10⁹
/// 	0 <= p <= (nums.length)/2
///

impl Solution {
    pub fn minimize_max(nums: Vec<i32>, p: i32) -> i32 {
        if p == 0 || nums.len() == 1 {
            return 0;
        }
        // To minimize the diffs between pairs, we sort the diffs in ascending order, and take the first p items
        // We also sort the pairs in an order, so that the diff between 2 adjacent element is the smallest
        let mut nums = nums;
        nums.sort();
        let diffs: Vec<i32> = nums
            .iter()
            .skip(1)
            .zip(nums.iter().take(nums.len() - 1))
            .map(|(a, b)| (a - b).abs())
            .collect();

        // We use a binary search approach to find the threshold where p pairs in nums have diff <= threshold
        let predicate = |threshold: &i32| -> bool {
            // Return true if it is in the first half, which is NOT able to form p pairs
            let mut p = p;
            let mut skip = false;
            for diff in &diffs {
                if skip || diff > threshold {
                    skip = false;
                    continue;
                }
                p -= 1;
                skip = true;
                if p == 0 {
                    break;
                }
            }
            return p > 0;
        };
        // Use binary search to find the first threshold where p pairs >= threshold
        let max = diffs
            .iter()
            .max()
            .expect("Line 38 makes sure that diffs is not empty");
        // Partition point expect the predicate to return true for the first half, and false for the second half
        // This value will hold the first index of the second half (or the length of the array), which is the first threshold where p pairs can be formed
        // let threshold = (0..=*max).collect::<Vec<i32>>().partition_point(predicate); // CANNOT use Rust's partition_point because it require allocated array (for random access)
        fn partition_point(left: i32, right: i32, predicate: impl Fn(&i32) -> bool) -> i32 {
            if left == right {
                return left;
            }
            let middle = (left + right) / 2;
            if predicate(&middle) {
                // move right
                partition_point(middle + 1, right, predicate)
            } else {
                // move left
                partition_point(left, middle, predicate)
            }
        }
        partition_point(0, *max, predicate)
    }
}
