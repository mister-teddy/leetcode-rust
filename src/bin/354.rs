/// ### 354. Russian Doll Envelopes
///
/// You are given a 2D array of integers envelopes where envelopes[i] = [wi, hi] represents the width and the height of an envelope.
///
/// One envelope can fit into another if and only if both the width and height of one envelope are greater than the other envelope's width and height.
///
/// Return the maximum number of envelopes you can Russian doll (i.e., put one inside the other).
///
/// Note: You cannot rotate an envelope.
///
///
/// Example 1:
/// - Input: envelopes = [[5,4],[6,4],[6,7],[2,3]]
/// - Output: 3
/// - Explanation: The maximum number of envelopes you can Russian doll is 3 ([2,3] => [5,4] => [6,7]).
///
/// Example 2:
/// - Input: envelopes = [[1,1],[1,1],[1,1]]
/// - Output: 1
///
/// Constraints:
/// - 1 <= envelopes.length <= 105
/// - envelopes[i].length == 2
/// - 1 <= wi, hi <= 105
struct Solution {}

fn main() {
    // assert_eq!(
    //     Solution::max_envelopes(vec![vec![5, 4], vec![6, 4], vec![6, 7], vec![2, 3]]),
    //     3
    // );
    // assert_eq!(
    //     Solution::max_envelopes(vec![vec![1, 1], vec![1, 1], vec![1, 1]]),
    //     1
    // );
    assert_eq!(
        Solution::max_envelopes(vec![
            vec![4, 5],
            vec![4, 6],
            vec![6, 7],
            vec![2, 3],
            vec![1, 1]
        ]),
        4
    );
}

impl Solution {
    fn find_index(dp: &[i32], target: &i32, prefix: usize) -> usize {
        let mid = dp.len() / 2;
        if dp[mid] == *target {
            return prefix + mid;
        }
        if dp[mid] > *target {
            if mid == 0 {
                return prefix;
            }
            return Self::find_index(&dp[..mid], target, prefix);
        }
        if mid == dp.len() - 1 {
            return prefix + mid + 1;
        }
        return Self::find_index(&dp[mid + 1..], target, prefix + mid + 1);
    }

    /// let dp[i] be the smallest posibile envelope size.1 that built the result with length i
    /// we have:
    /// - dp[0] = non sense => since we have at least 1 envelop, there is no solution such that length is 0
    /// - dp[1] = any envelop
    /// - dp[i] = the i(th) smallest envelope size.1
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        envelopes.sort_by(|a, b| match a[0].cmp(&b[0]) {
            std::cmp::Ordering::Equal => b[1].cmp(&a[1]),
            ord => ord,
        });
        let mut dp = vec![i32::MAX; envelopes.len() + 1];
        dp[0] = 0;
        let mut res = 1;
        for envelop in envelopes {
            let h = envelop[1];
            let index = Self::find_index(&dp, &h, 0);
            dp[index] = h;
            if index == dp.len() - 1 {
                return index as i32;
            }
            res = res.max(index);
        }
        res as i32
    }
}
