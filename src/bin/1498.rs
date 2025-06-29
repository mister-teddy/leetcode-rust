const MODULO: i32 = 1_000_000_007;
/// Category: algorithms
/// Level: Medium
/// Percent: 43.729652%

/// You are given an array of integers nums and an integer target.
///
/// Return the number of non-empty subsequences of nums such that the sum of the minimum and maximum element on it is less or equal to target. Since the answer may be too large, return it modulo 10⁹ + 7.
///
///
/// Example 1:
///
/// Input: nums = [3,5,6,7], target = 9
/// Output: 4
/// Explanation: There are 4 subsequences that satisfy the condition.
/// [3] -> Min value + max value <= target (3 + 3 <= 9)
/// [3,5] -> (3 + 5 <= 9)
/// [3,5,6] -> (3 + 6 <= 9)
/// [3,6] -> (3 + 6 <= 9)
///
///
/// Example 2:
///
/// Input: nums = [3,3,6,8], target = 10
/// Output: 6
/// Explanation: There are 6 subsequences that satisfy the condition. (nums can have repeated numbers).
/// [3] , [3] , [3,3], [3,6] , [3,6] , [3,3,6]
///
///
/// Example 3:
///
/// Input: nums = [2,3,3,4,6,7], target = 12
/// Output: 61
/// Explanation: There are 63 non-empty subsequences, two of them do not satisfy the condition ([6,7], [7]).
/// Number of valid subsequences (63 - 2 = 61).
///
///
///
/// Constraints:
///
///
/// 	1 <= nums.length <= 10⁵
/// 	1 <= nums[i] <= 10⁶
/// 	1 <= target <= 10⁶
impl Solution {
    pub fn num_subseq(nums: Vec<i32>, target: i32) -> i32 {
        // We can solve this problem using O(nlogn) sort, then 2 pointers to sum all posibilities between possible ranges
        let mut sorted = nums;
        sorted.sort();

        let mut res = 0;
        let mut right = sorted.len() - 1;
        let mut left = 0;
        loop {
            while left < right && sorted[left + 1] + sorted[right] <= target {
                left += 1;
            }

            if sorted[left] + sorted[right] <= target {
                // Any index between 0..=left can be the min, because it will keep the sum <= target
                // As long as we keep min and max valid, the absence of any element(s) in between does not matter
                // So the count of possibilities = 2^(how_many_numbers_in_between), for each min & max
                let start = left.min(right);
                let end = right;
                // for i in 0..=start {
                //     let count = 2i32.pow((end - i).saturating_sub(1));
                //     res += count;
                // }

                // The above calculation takes O(n) in the worst case, which extend our program complexity to O(n^2), which will cause time limit exceeded.
                // We must find a way to calculate count in O(1)
                // We are trying to calculate:
                // 2^(end - 1) + 2^(end - 2) + ... + 2^(end - start - 1)
                // = 2^(end - start - 1)*[2^(end - 1 - (end - start - 1) + ... + 2^2 + 2^1 + 2^0]
                // We can reduce 2^0 + 2^1 + 2^2 + ... + 2^n = 2^(n+1) - 1
                // So the above formula becomes:
                // = 2^(end - start - 1)*[2^(end - 1 - end + start + 1 + 1) - 1]
                // = 2^(end - start - 1)*[2^(start + 1) - 1]
                //
                // However, for these cases where start == end
                // For example start == end == 3
                // = 2^(3-1) + 2^(3-2) + 2^(3-3) + 2^(3-4) <= Two 2^(0) here
                // So the result become: 1 + 1 + 2^1 + 2^2 + ... + 2^(end - 1) = 1 + 2^end - 1 = 2^end
                let count = if end == start {
                    2i64.mod_pow(end)
                } else {
                    (2i64.mod_pow(start + 1) - 1) * 2i64.mod_pow((end - start).saturating_sub(1))
                        % MODULO as i64
                } as i32;
                res = (res + count) % MODULO;
                // println!("{start} -> {end} = {count}");
            }

            // next iteration
            if right == 0 {
                break;
            }
            right -= 1;
        }

        res
    }
}

trait ModPow {
    fn mod_pow(self, b: usize) -> i64;
}

impl ModPow for i64 {
    // Return a^b % MODULO
    fn mod_pow(self, b: usize) -> i64 {
        if b == 0 {
            return 1;
        }
        // a^b = (a^(b/2))^2
        // However, if b is odd, then a^b = a*a^(b-1)
        if b % 2 == 1 {
            return (self * self.mod_pow(b - 1)) % MODULO as i64;
        }
        let res = self.mod_pow(b / 2);
        return res * res % MODULO as i64;
    }
}
