/// Category: algorithms
/// Level: Easy
/// Percent: 76.70181%

/// Given an integer numRows, return the first numRows of Pascal's triangle.
///
/// In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:
///
///
/// Example 1:
/// Input: numRows = 5
/// Output: [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]
/// Example 2:
/// Input: numRows = 1
/// Output: [[1]]
///
///
/// Constraints:
///
///
/// 	1 <= numRows <= 30
///
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        if num_rows == 1 {
            return vec![vec![1]];
        }

        let mut res = Solution::generate(num_rows - 1);
        let prev_row = res.last().unwrap();
        let mut last_row = vec![1];
        for i in 0..prev_row.len() - 1 {
            last_row.push(prev_row[i] + prev_row[i + 1]);
        }
        last_row.push(1);
        res.push(last_row);
        res
    }
}
