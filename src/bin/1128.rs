/// Category: algorithms
/// Level: Easy
/// Percent: 49.125023%

/// Given a list of dominoes, dominoes[i] = [a, b] is equivalent to dominoes[j] = [c, d] if and only if either (a == c and b == d), or (a == d and b == c) - that is, one domino can be rotated to be equal to another domino.
///
/// Return the number of pairs (i, j) for which 0 <= i < j < dominoes.length, and dominoes[i] is equivalent to dominoes[j].
///
///
/// Example 1:
///
/// Input: dominoes = [[1,2],[2,1],[3,4],[5,6]]
/// Output: 1
///
///
/// Example 2:
///
/// Input: dominoes = [[1,2],[1,2],[1,1],[1,2],[2,2]]
/// Output: 3
///
///
///
/// Constraints:
///
///
/// 	1 <= dominoes.length <= 4 * 10⁴
/// 	dominoes[i].length == 2
/// 	1 <= dominoes[i][j] <= 9
///

impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut count = vec![0; 100];
        for dom in dominoes {
            let val = if dom[0] < dom[1] {
                dom[0] * 10 + dom[1]
            } else {
                dom[1] * 10 + dom[0]
            };
            count[val as usize] += 1;
        }
        count.iter().fold(0, |res, x| res + x * (x - 1) / 2)
    }
}
