use std::cmp;

pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
    let mut dp = matrix[0].clone();
    let mut backup = dp[0];
    for row in &matrix[1..] {
        for i in 0..row.len() {
            let mut min = dp[i];
            if i > 0 {
                min = cmp::min(min, backup);
            }
            if i < row.len() - 1 {
                min = cmp::min(min, dp[i + 1]);
            }
            let tmp = row[i] + min;
            backup = dp[i];
            dp[i] = tmp;
        }
    }
    dp.iter().min().unwrap().clone()
}

fn main() {
    // [[2,1,3],[6,5,4],[7,8,9]]
    let res = min_falling_path_sum(vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]]);
    print!("{res}");
}
