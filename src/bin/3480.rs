/// Category: algorithms
/// Level: Hard
/// Percent: 31.987408%

/// You are given an integer n which represents an array nums containing the numbers from 1 to n in order. Additionally, you are given a 2D array conflictingPairs, where conflictingPairs[i] = [a, b] indicates that a and b form a conflicting pair.
///
/// Remove exactly one element from conflictingPairs. Afterward, count the number of non-empty subarrays of nums which do not contain both a and b for any remaining conflicting pair [a, b].
///
/// Return the maximum number of subarrays possible after removing exactly one conflicting pair.
///
///
/// Example 1:
///
///
/// Input: n = 4, conflictingPairs = [[2,3],[1,4]]
///
/// Output: 9
///
/// Explanation:
///
///
/// 	Remove [2, 3] from conflictingPairs. Now, conflictingPairs = [[1, 4]].
/// 	There are 9 subarrays in nums where [1, 4] do not appear together. They are [1], [2], [3], [4], [1, 2], [2, 3], [3, 4], [1, 2, 3] and [2, 3, 4].
/// 	The maximum number of subarrays we can achieve after removing one element from conflictingPairs is 9.
///
///
///
/// Example 2:
///
///
/// Input: n = 5, conflictingPairs = [[1,2],[2,5],[3,5]]
///
/// Output: 12
///
/// Explanation:
///
///
/// 	Remove [1, 2] from conflictingPairs. Now, conflictingPairs = [[2, 5], [3, 5]].
/// 	There are 12 subarrays in nums where [2, 5] and [3, 5] do not appear together.
/// 	The maximum number of subarrays we can achieve after removing one element from conflictingPairs is 12.
///
///
///
///
/// Constraints:
///
///
/// 	2 <= n <= 10⁵
/// 	1 <= conflictingPairs.length <= 2 * n
/// 	conflictingPairs[i].length == 2
/// 	1 <= conflictingPairs[i][j] <= n
/// 	conflictingPairs[i][0] != conflictingPairs[i][1]
///

impl Solution {
    pub fn max_subarrays(n: i32, conflicting_pairs: Vec<Vec<i32>>) -> i64 {
        // Let's solve this problem brutefotely
        let mut res = 0;
        for i in 0..conflicting_pairs.len() {
            let mut without_i = conflicting_pairs.clone();
            without_i.remove(i);
            res = res.max(count_subarrays(n, without_i));
        }
        res
    }
}

fn count_subarrays(n: i32, conflicting_pairs: Vec<Vec<i32>>) -> i64 {
    // Ensure for each pair, the larger element is on the right
    let pairs: Vec<Vec<i32>> = conflicting_pairs
        .iter()
        .map(|pair| {
            if pair[0] > pair[1] {
                vec![pair[1], pair[0]]
            } else {
                pair.clone()
            }
        })
        .collect();
    // Let's count how many subarrays without conflicting
    let mut count = 0i64;
    // by summing subarrays with each left endpoint
    for left in 1..=n {
        // The number of subarray starting from left is right - left + 1
        // Now we just have to figure out what is right based on the conflicting_pairs
        // If the pair[0] is smaller than left, we can freely pick any number without being fear of conflict
        let pairs_0_passed_left = pairs.iter().filter(|pair| pair[0] >= left);
        // Otherwise, we just pick the smaller one, and stop at right before pair[1]
        let min_pair_1 = pairs_0_passed_left.map(|pair| pair[1]).min();
        let right = min_pair_1.map_or(n, |min| min - 1);
        count += (right - left + 1) as i64;
    }
    count
}
