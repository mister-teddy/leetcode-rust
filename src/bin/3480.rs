use std::{cmp::Reverse, collections::BinaryHeap, ops::Index};

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
        // Instead of solving it bruteforcely, we can:
        // - Count the result without removing any pair
        // - Because for each pair removed, the result ALWAYS INCREASES (but by how much?)
        // So if we know the max gain, we can answer the problem

        // So we store the gains inside one array. We try to calculate this array while we count the number of subarrays
        let mut gains = vec![0i64; conflicting_pairs.len()];

        let count_without_remove = count_subarrays(n, conflicting_pairs, &mut gains);
        let max_gain = gains.iter().max().unwrap();
        count_without_remove + max_gain
    }
}

fn count_subarrays(n: i32, conflicting_pairs: Vec<Vec<i32>>, gains: &mut Vec<i64>) -> i64 {
    // (After many improvements...)
    // Let's use a heap so that we can quickly pop out the smallest cap to process incrementally
    // While building the heap, we ensure for each pair, the larger element is on the left
    let mut heap = BinaryHeap::new();
    for (index, pair) in conflicting_pairs.iter().enumerate() {
        if pair[0] > pair[1] {
            heap.push(Reverse((pair[0], pair[1], index))); // And store the pair's original index inside the tuple so that we can assign back to `gains`
        } else {
            heap.push(Reverse((pair[1], pair[0], index)));
        }
    }

    let mut count = 0i64;
    // We process each left end
    for left in 1..=n {
        let mut right = n;
        // We only care about pairs whose smaller end is >= left (read the commit history for the reason why)
        while heap.peek().is_some_and(|Reverse(pair)| pair.1 < left) {
            heap.pop();
        }
        if let Some(Reverse(pair)) = heap.pop() {
            right = pair.0 - 1;
            // Now we find the next valid pair, to calculate the gain if pair is removed ("luckily", this problem only removes 1 pair)
            while heap.peek().is_some_and(|Reverse(pair)| pair.1 < left) {
                heap.pop(); // We pop out outdated pairs to prevent a nested linear loop
            }
            let next_right = heap.peek().map(|Reverse(pair)| pair.0 - 1).unwrap_or(n);
            heap.push(Reverse(pair)); // Push back peek
                                      // If `pair` were to be removed, its gain would be the difference between next_right and right
            gains[pair.2] += (next_right - right) as i64;
        }
        // The number of subarrays is how many items between left and right, inclusively
        count += (right - left + 1) as i64;
    }
    count
}
