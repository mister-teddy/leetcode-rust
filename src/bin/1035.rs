/// ### 1035. Uncrossed Lines
///
/// You are given two integer arrays nums1 and nums2. We write the integers of nums1 and nums2 (in the order they are given) on two separate horizontal lines.
/// We may draw connecting lines: a straight line connecting two numbers nums1[i] and nums2[j] such that:
/// nums1[i] == nums2[j], and the line we draw does not intersect any other connecting (non-horizontal) line.
/// Note that a connecting line cannot intersect even at the endpoints (i.e., each number can only belong to one connecting line).
/// Return the maximum number of connecting lines we can draw in this way.
///
/// Example 1:
/// Input: nums1 = [1,4,2], nums2 = [1,2,4]
/// Output: 2
/// Explanation: We can draw 2 uncrossed lines as in the diagram.
/// We cannot draw 3 uncrossed lines, because the line from nums1[1] = 4 to nums2[2] = 4 will intersect the line from nums1[2]=2 to nums2[1]=2.
///
/// Example 2:
/// Input: nums1 = [2,5,1,2,5], nums2 = [10,5,2,1,5,2]
/// Output: 3
///
/// Example 3:
/// Input: nums1 = [1,3,7,1,7,5], nums2 = [1,9,2,5,1]
/// Output: 2
///
/// Constraints:
/// 1 <= nums1.length, nums2.length <= 500
/// 1 <= nums1[i], nums2[j] <= 2000
struct Solution {}

fn main() {
    let nums1 = vec![1, 4, 2];
    let nums2 = vec![1, 2, 4];
    assert_eq!(Solution::max_uncrossed_lines(nums1, nums2), 2);

    let nums1 = vec![2, 5, 1, 2, 5];
    let nums2 = vec![10, 5, 2, 1, 5, 2];
    assert_eq!(Solution::max_uncrossed_lines(nums1, nums2), 3);

    let nums1 = vec![1, 3, 7, 1, 7, 5];
    let nums2 = vec![1, 9, 2, 5, 1];
    assert_eq!(Solution::max_uncrossed_lines(nums1, nums2), 2);

    let nums1 = vec![1];
    let nums2 = vec![2000];
    assert_eq!(Solution::max_uncrossed_lines(nums1, nums2), 0);

    let nums1 = vec![2000];
    let nums2 = vec![2000];
    assert_eq!(Solution::max_uncrossed_lines(nums1, nums2), 1);
}

impl Solution {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        // We can solve this problem with dynamic programming:
        // let dp[i][j] be the maximum number of connecting lines we can draw from nums1[0..i) to nums[0..j)
        // it is obvious that dp[0][*] = dp[*][0] = 0 (no line drawn)
        // let mut dp = vec![vec![0; nums2.len() + 1]; nums1.len() + 1];

        // R1: We can further improve the space usage by storing only the last row
        let mut dp = vec![0; nums2.len() + 1];

        // When we calculating dp[i][j], we already have the correct result for dp[i][0..j-1] dp[0..i-1][*]
        // So we only focus on the last 2 numbers: nums1[i-1] & nums2[j-1]
        // If these two numbers are equal, we can connect them, and the result of dp[i][j] would be 1 + the result where these 2 nums are not considered (dp[i-1][j-1]). By this logic, the "no intersecting rule" will also be adhered.
        // Otherwise, let's take the max result of either dp[i-1][j] or dp[i][j-1] (matching other numbers)
        // for i in 1..=nums1.len() {
        //     for j in 1..=nums2.len() {
        //         if nums1[i - 1] == nums2[j - 1] {
        //             dp[i][j] = dp[i - 1][j - 1] + 1;
        //         } else {
        //             dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
        //         }
        //     }
        // }

        for i in 1..=nums1.len() {
            // R1: Because `dp[i - 1][j - 1]` would be overwritten in the previous iteration, we need a variable to save its value
            let mut prev = 0;
            for j in 1..=nums2.len() {
                let val = if nums1[i - 1] == nums2[j - 1] {
                    // R1: `prev` is the equivalent of `dp[i-1][j-1]``
                    prev + 1
                } else {
                    // R1: `dp[j]` is the equivalent of `dp[i-1][j]`, and `dp[j-1]` is the equivalent of `dp[i][j-1]`
                    dp[j].max(dp[j - 1])
                };
                prev = dp[j];
                dp[j] = val;
            }
        }

        dp[nums2.len()]
    }
}
