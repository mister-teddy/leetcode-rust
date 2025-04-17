use std::collections::HashMap;

/// Category: algorithms
/// Level: Medium
/// Percent: 52.928776%

/// Given an integer array nums and an integer k, return the number of good subarrays of nums.
///
/// A subarray arr is good if there are at least k pairs of indices (i, j) such that i < j and arr[i] == arr[j].
///
/// A subarray is a contiguous non-empty sequence of elements within an array.
///
///
/// Example 1:
///
/// Input: nums = [1,1,1,1,1], k = 10
/// Output: 1
/// Explanation: The only good subarray is the array nums itself.
///
///
/// Example 2:
///
/// Input: nums = [3,1,4,3,2,2,4], k = 2
/// Output: 4
/// Explanation: There are 4 different good subarrays:
/// - [3,1,4,3,2,2] that has 2 pairs.
/// - [3,1,4,3,2,2,4] that has 3 pairs.
/// - [1,4,3,2,2,4] that has 2 pairs.
/// - [4,3,2,2,4] that has 2 pairs.
///
///
///
/// Constraints:
///
///
/// 	1 <= nums.length <= 10⁵
/// 	1 <= nums[i], k <= 10⁹
///

impl Solution {
    pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
        // Let's solve this problem using sliding window
        let mut start = 0;
        let mut end = 0; // exclusive

        // Store occurence count of each number inside a bag
        let mut bag = HashMap::new();
        let mut res = 0;
        let mut total = 0;
        loop {
            update_bag(&mut bag, nums[end], 1, &mut total);
            loop {
                // println!("{}->{}", start, end);
                let good = total >= k as i64;
                if good {
                    res += nums.len() as i64 - end as i64;
                    update_bag(&mut bag, nums[start], -1, &mut total);
                    start += 1;
                } else {
                    break;
                }
            }
            end += 1;
            if end == nums.len() {
                break;
            }
        }

        res
    }
}

fn update_bag(bag: &mut HashMap<i32, usize>, key: i32, change: i32, total: &mut i64) {
    let count = bag.entry(key).or_insert(0);
    let old_sum = *count * (*count - 1) / 2;
    *count += change as usize;
    let new_sum = *count * (*count - 1) / 2;
    *total += (new_sum - old_sum) as i64;
}
