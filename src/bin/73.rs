/// Category: algorithms
/// Level: Medium
/// Percent: 59.364002%

/// Given an m x n integer matrix matrix, if an element is 0, set its entire row and column to 0's.
///
/// You must do it in place.
///
///
/// Example 1:
///
/// Input: matrix = [[1,1,1],[1,0,1],[1,1,1]]
/// Output: [[1,0,1],[0,0,0],[1,0,1]]
///
///
/// Example 2:
///
/// Input: matrix = [[0,1,2,0],[3,4,5,2],[1,3,1,5]]
/// Output: [[0,0,0,0],[0,4,5,0],[0,3,1,0]]
///
///
///
/// Constraints:
///
///
/// 	m == matrix.length
/// 	n == matrix[0].length
/// 	1 <= m, n <= 200
/// 	-2³¹ <= matrix[i][j] <= 2³¹ - 1
///
///
///
/// Follow up:
///
///
/// 	A straightforward solution using O(mn) space is probably a bad idea.
/// 	A simple improvement uses O(m + n) space, but still not the best solution.
/// 	Could you devise a constant space solution?

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let top = matrix[0].iter().any(|&x| x == 0);
        let mut left = false;
        for i in 0..matrix.len() {
            if matrix[i][0] == 0 {
                left = true;
            }
            for j in 0..matrix[i].len() {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }
        for i in (1..matrix.len()).rev() {
            for j in (1..matrix[i].len()).rev() {
                if matrix[0][j] == 0 || matrix[i][0] == 0 {
                    matrix[i][j] = 0;
                }
            }
            if left {
                matrix[i][0] = 0;
            }
        }
        if top {
            matrix[0].fill(0);
        }
    }
}
