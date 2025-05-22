/// Category: algorithms
/// Level: Medium
/// Percent: 26.09967%

/// You are given an integer array nums of length n and a 2D array queries where queries[i] = [li, ri].
///
/// Each queries[i] represents the following action on nums:
///
///
/// 	Decrement the value at each index in the range [li, ri] in nums by at most 1.
/// 	The amount by which the value is decremented can be chosen independently for each index.
///
///
/// A Zero Array is an array with all its elements equal to 0.
///
/// Return the maximum number of elements that can be removed from queries, such that nums can still be converted to a zero array using the remaining queries. If it is not possible to convert nums to a zero array, return -1.
///
///
/// Example 1:
///
///
/// Input: nums = [2,0,2], queries = [[0,2],[0,2],[1,1]]
///
/// Output: 1
///
/// Explanation:
///
/// After removing queries[2], nums can still be converted to a zero array.
///
///
/// 	Using queries[0], decrement nums[0] and nums[2] by 1 and nums[1] by 0.
/// 	Using queries[1], decrement nums[0] and nums[2] by 1 and nums[1] by 0.
///
///
///
/// Example 2:
///
///
/// Input: nums = [1,1,1,1], queries = [[1,3],[0,2],[1,3],[1,2]]
///
/// Output: 2
///
/// Explanation:
///
/// We can remove queries[2] and queries[3].
///
///
/// Example 3:
///
///
/// Input: nums = [1,2,3,4], queries = [[0,3]]
///
/// Output: -1
///
/// Explanation:
///
/// nums cannot be converted to a zero array even after using all the queries.
///
///
///
/// Constraints:
///
///
/// 	1 <= nums.length <= 10⁵
/// 	0 <= nums[i] <= 10⁵
/// 	1 <= queries.length <= 10⁵
/// 	queries[i].length == 2
/// 	0 <= li <= ri < nums.length
///
use std::collections::BinaryHeap;

impl Solution {
    pub fn max_removal(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        // We can solve this problem with greedy
        // Let a cursor moves forward along nums

        // In order to reduce `num` down to 0, we need to pick `num` queries
        // In order to minimise the number of queries needed, we will prioritise valid queries (queries that cover num) but span the furthest (so that it reduce more numbers in the future)
        // To efficient select these queries, we maintain max heap of valid queries (*)
        let mut valid_queries = BinaryHeap::new();
        // To keep the heap valid, we will only insert queries into it when we meet the query start point
        // So we will need to sort the queries in increasing start point order
        let mut queries = queries;
        queries.sort();
        let mut q = 0;
        // For efficient range reduction, we will store use a prefix sum approach like we did in the previous solution
        let mut reduction = 0;
        let mut changes = vec![0; nums.len() + 1];
        for (i, &num) in nums.iter().enumerate() {
            // Prefix sum approach
            reduction -= changes[i];
            while q < queries.len() && queries[q][0] as usize == i {
                // Push valid queries into the heap
                valid_queries.push(queries[q][1]);
                q += 1;
            }
            while reduction < num {
                // Pop the furthest query
                if valid_queries.is_empty() {
                    // If there is nothing to pop, that means we cannot reduce num to 0
                    return -1;
                }
                let end = valid_queries.pop().unwrap();
                // If the best query we can find is an expired query, then we cannot reduce num to 0 either
                if (end as usize) < i {
                    return -1;
                }
                // Prefix sum approach
                reduction += 1;
                changes[end as usize + 1] += 1;
            }
        }
        // In the end, the result of this problem is how many queries are there left in the heap
        valid_queries.len() as i32
    }
}
