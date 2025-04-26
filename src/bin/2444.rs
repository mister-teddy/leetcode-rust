use std::collections::HashMap;

/// Category: algorithms
/// Level: Hard
/// Percent: 67.68362%

/// You are given an integer array nums and two integers minK and maxK.
///
/// A fixed-bound subarray of nums is a subarray that satisfies the following conditions:
///
///
/// 	The minimum value in the subarray is equal to minK.
/// 	The maximum value in the subarray is equal to maxK.
///
///
/// Return the number of fixed-bound subarrays.
///
/// A subarray is a contiguous part of an array.
///
///
/// Example 1:
///
/// Input: nums = [1,3,5,2,7,5], minK = 1, maxK = 5
/// Output: 2
/// Explanation: The fixed-bound subarrays are [1,3,5] and [1,3,5,2].
///
///
/// Example 2:
///
/// Input: nums = [1,1,1,1], minK = 1, maxK = 1
/// Output: 10
/// Explanation: Every subarray of nums is a fixed-bound subarray. There are 10 possible subarrays.
///
///
///
/// Constraints:
///
///
/// 	2 <= nums.length <= 10⁵
/// 	1 <= nums[i], minK, maxK <= 10⁶
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        // We can solve this problem using sliding window
        let mut positions = vec![];
        for i in 0..nums.len() {
            if nums[i] <= min_k || nums[i] >= max_k {
                positions.push(i);
            }
        }
        if positions.is_empty() {
            return 0;
        }
        let mut left_invalid_points = vec![-1i64; nums.len() + 1];
        for i in 1..=nums.len() {
            if nums[i - 1] > max_k || nums[i - 1] < min_k {
                left_invalid_points[i] = i as i64 - 1;
            } else {
                left_invalid_points[i] = left_invalid_points[i - 1];
            }
        }
        let mut right_invalid_points = vec![nums.len(); nums.len() + 1];
        for i in (0..nums.len()).rev() {
            if nums[i] > max_k || nums[i] < min_k {
                right_invalid_points[i] = i;
            } else {
                right_invalid_points[i] = right_invalid_points[i + 1];
            }
        }
        let mut res = 0i64;
        let mut start_iter = positions.iter();
        let mut end_iter = positions.iter();
        let mut prev = -1;
        let mut start = *start_iter.next().unwrap();
        let mut end = 0;
        let mut counts = HashMap::new();
        while end < nums.len() {
            if let Some(next) = end_iter.next() {
                end = *next;
                if nums[end] <= min_k || nums[end] >= max_k {
                    *counts.entry(nums[end]).or_insert(0) += 1;
                }
                while start <= end {
                    let valid = counts.len() == if min_k == max_k { 1 } else { 2 }
                        && *counts.get(&min_k).unwrap_or(&0) > 0
                        && *counts.get(&max_k).unwrap_or(&0) > 0;
                    if valid {
                        let prefix_count = start as i64 - prev.max(left_invalid_points[start]);
                        let suffix_count = right_invalid_points[end] - end;
                        res += suffix_count as i64 * prefix_count;
                        let entry = counts.entry(nums[start]).or_insert(0);
                        *entry -= 1;
                        if *entry <= 0 {
                            counts.remove(&nums[start]);
                        }
                        let next = match start_iter.next() {
                            Some(x) => *x,
                            None => nums.len(),
                        };
                        prev = start as i64;
                        start = next;
                    } else {
                        break;
                    }
                }
            } else {
                break;
            }
        }
        res
    }
}

struct Solution {}

fn main() {
    // assert_eq!(
    //     Solution::count_subarrays(
    //         vec![
    //             8121, 8121, 955792, 925378, 383928, 673920, 457030, 925378, 925378, 925378, 92893,
    //             456370, 925378
    //         ],
    //         8121,
    //         925378
    //     ),
    //     0
    // );
    assert_eq!(Solution::count_subarrays(vec![4, 3], 3, 3), 1);
}
