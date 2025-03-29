use std::collections::{BinaryHeap, HashSet};

/// 2818. Apply Operations to Maximize Score
///
/// You are given an array nums of n positive integers and an integer k.
///
/// Initially, you start with a score of 1. You have to maximize your score by applying the following operation at most k times:
///
/// - Choose any non-empty subarray nums[l, ..., r] that you haven't chosen previously.
/// - Choose an element x of nums[l, ..., r] with the highest prime score. If multiple such elements exist, choose the one with the smallest index.
/// - Multiply your score by x.
///
/// Here, nums[l, ..., r] denotes the subarray of nums starting at index l and ending at the index r, both ends being inclusive.
///
/// The prime score of an integer x is equal to the number of distinct prime factors of x. For example, the prime score of 300 is 3 since 300 = 2 * 2 * 3 * 5 * 5.
///
/// Return the maximum possible score after applying at most k operations.
///
/// Since the answer may be large, return it modulo 10^9 + 7.
///
/// # Example 1:
///
/// Input: nums = [8,3,9,3,8], k = 2  
/// Output: 81  
/// Explanation: To get a score of 81, we can apply the following operations:  
/// - Choose subarray nums[2, ..., 2]. nums[2] is the only element in this subarray. Hence, we multiply the score by nums[2]. The score becomes 1 * 9 = 9.  
/// - Choose subarray nums[2, ..., 3]. Both nums[2] and nums[3] have a prime score of 1, but nums[2] has the smaller index. Hence, we multiply the score by nums[2]. The score becomes 9 * 9 = 81.  
/// It can be proven that 81 is the highest score one can obtain.
///
/// # Example 2:
///
/// Input: nums = [19,12,14,6,10,18], k = 3  
/// Output: 4788  
/// Explanation: To get a score of 4788, we can apply the following operations:  
/// - Choose subarray nums[0, ..., 0]. nums[0] is the only element in this subarray. Hence, we multiply the score by nums[0]. The score becomes 1 * 19 = 19.  
/// - Choose subarray nums[5, ..., 5]. nums[5] is the only element in this subarray. Hence, we multiply the score by nums[5]. The score becomes 19 * 18 = 342.  
/// - Choose subarray nums[2, ..., 3]. Both nums[2] and nums[3] have a prime score of 2, but nums[2] has the smaller index. Hence, we multiply the score by nums[2]. The score becomes 342 * 14 = 4788.  
/// It can be proven that 4788 is the highest score one can obtain.
///
/// # Constraints:
///
/// - 1 <= nums.length == n <= 10^5  
/// - 1 <= nums[i] <= 10^5  
/// - 1 <= k <= min(n * (n + 1) / 2, 10^9)
struct Solution {}

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        // First, we calculate the prime score of each number
        // For a faster calculation, we will store a set of unique prime factors, for all number between 1 and max
        let max = *nums.iter().max().unwrap() as usize; // O(n)
        let mut prime_factors = vec![HashSet::new(); max + 1];
        for i in 2..=max {
            for k in 2..=((i as f32).sqrt() as usize) {
                if i % k == 0 {
                    for factor in prime_factors[k].clone() {
                        prime_factors[i].insert(factor);
                    }
                    for factor in prime_factors[i / k].clone() {
                        prime_factors[i].insert(factor);
                    }
                    break;
                }
            }
            if prime_factors[i].is_empty() {
                prime_factors[i].insert(i);
            }
        } // O(max(nums)^1.5)

        // Second, we calculate the prime score of each number
        let prime_scores: Vec<usize> = nums
            .iter()
            .map(|num| prime_factors[*num as usize].len())
            .collect(); // O(n)

        // Third, we calculate how much a number can "span" left or right
        // Let spans[i] = (left, right), where:
        //   - left is how many numbers to the left that have prime score < nums[i]
        //   - right is how many numbers to the right that have prime score <= nums[i]
        let mut spans = Vec::new();
        for i in 0..nums.len() {
            let mut left = 0;
            while i - left > 0 && prime_scores[i - left - 1] < prime_scores[i] {
                left += 1;
            }
            let mut right = 0;
            while i + right < nums.len() - 1 && prime_scores[i + right + 1] <= prime_scores[i] {
                right += 1;
            }
            spans.push((left, right));
        } // O(n) on average

        // Four, If a number x can span left and right, we can pick a combination of (left + 1)*(right + 1) ranges,
        // in which x still be the number with the most prime score
        // So we store a max heap with x as the priority metric, along with its total combinations
        let mut heap = BinaryHeap::new();
        for i in 0..nums.len() {
            let (left, right) = spans[i];
            heap.push((nums[i], (left + 1) * (right + 1)));
        } // O(nlogn)

        // Finally, we return at most k combinations from the heap, multiplied to get the result
        let mut total = 0;
        let mut result = 1 as u64;
        const MODULO: u64 = 1_000_000_000 + 7;
        while let Some((x, count)) = heap.pop() {
            let pick = count.min(k as usize - total);
            for _ in 0..pick {
                result = (result * (x as u64)) % MODULO;
            }
            total += pick;
            if total == k as usize {
                break;
            }
        } // O(min(k, nlogn))

        result as i32
    }
}

fn main() {
    // Example 1
    assert_eq!(Solution::maximum_score(vec![8, 3, 9, 3, 8], 2), 81);

    // Example 2
    assert_eq!(
        Solution::maximum_score(vec![19, 12, 14, 6, 10, 18], 3),
        4788
    );
    // Test case 337
    assert_eq!(
        Solution::maximum_score(vec![3289, 2832, 14858, 22011], 6),
        256720975
    );
}
