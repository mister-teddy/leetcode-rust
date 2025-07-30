use std::collections::VecDeque;

/// Category: algorithms
/// Level: Medium
/// Percent: 44.95067%

/// You are given a 0-indexed array nums of length n, consisting of non-negative integers. For each index i from 0 to n - 1, you must determine the size of the minimum sized non-empty subarray of nums starting at i (inclusive) that has the maximum possible bitwise OR.
///
///
/// 	In other words, let Bij be the bitwise OR of the subarray nums[i...j]. You need to find the smallest subarray starting at i, such that bitwise OR of this subarray is equal to max(Bik) where i <= k <= n - 1.
///
///
/// The bitwise OR of an array is the bitwise OR of all the numbers in it.
///
/// Return an integer array answer of size n where answer[i] is the length of the minimum sized subarray starting at i with maximum bitwise OR.
///
/// A subarray is a contiguous non-empty sequence of elements within an array.
///
///
/// Example 1:
///
/// Input: nums = [1,0,2,1,3]
/// Output: [3,3,2,2,1]
/// Explanation:
/// The maximum possible bitwise OR starting at any index is 3.
/// - Starting at index 0, the shortest subarray that yields it is [1,0,2].
/// - Starting at index 1, the shortest subarray that yields the maximum bitwise OR is [0,2,1].
/// - Starting at index 2, the shortest subarray that yields the maximum bitwise OR is [2,1].
/// - Starting at index 3, the shortest subarray that yields the maximum bitwise OR is [1,3].
/// - Starting at index 4, the shortest subarray that yields the maximum bitwise OR is [3].
/// Therefore, we return [3,3,2,2,1].
///
///
/// Example 2:
///
/// Input: nums = [1,2]
/// Output: [2,1]
/// Explanation:
/// Starting at index 0, the shortest subarray that yields the maximum bitwise OR is of length 2.
/// Starting at index 1, the shortest subarray that yields the maximum bitwise OR is of length 1.
/// Therefore, we return [2,1].
///
///
///
/// Constraints:
///
///
/// 	n == nums.length
/// 	1 <= n <= 10⁵
/// 	0 <= nums[i] <= 10⁹
///

impl Solution {
    pub fn smallest_subarrays(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res = vec![1; n];

        // The matrix should be nxm, where m is the length of the binary presentation of the largest number in nums
        let max = nums.iter().max().expect("n >= 1");
        if *max == 0 {
            // You cannot take ilog2 of 0, so we handle this special case
            return res;
        }
        let m = max.ilog2() as usize + 1;
        // The last column should be all n
        let mut suffixes = vec![vec![n; m]; n + 1];

        for (i, num) in nums.iter().enumerate().rev() {
            for (j, bit) in to_bin(*num, m).iter().enumerate() {
                // To answer the question, we must know the nearest index of the next bit "1" to the right
                // So we count from the right to the left, if the current bit is "1", we store its index
                // If the current bit is "0", we "copy" the data on the right
                suffixes[i][j] = if *bit { i } else { suffixes[i + 1][j] }
            }

            // Then the largest indexes of all the "1" bits found the nearest location where we can stop
            let nearest_index = suffixes[i].iter().filter(|&&x| x < n).max().unwrap_or(&i);
            // The answer is the length, not the nearest index though
            res[i] = (nearest_index - i + 1) as i32;
        }
        res
    }
}

/// This function return the binary presentation of a number as a fixed size array
fn to_bin(num: i32, m: usize) -> Vec<bool> {
    let mut num = num;
    let mut i = m - 1;
    let mut bits = vec![false; m];
    while num > 0 {
        bits[i] = num % 2 != 0;
        i -= 1;
        num /= 2;
    }
    bits.into()
}
