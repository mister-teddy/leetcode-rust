/// # 2140. Solving Questions With Brainpower
///
/// You are given a 0-indexed 2D integer array questions where questions[i] = [pointsi, brainpoweri].
/// The array describes the questions of an exam, where you have to process the questions in order (i.e., starting from question 0) and make a decision whether to solve or skip each question. Solving question i will earn you pointsi points but you will be unable to solve each of the next brainpoweri questions. If you skip question i, you get to make the decision on the next question.
/// For example, given questions = [[3, 2], [4, 3], [4, 4], [2, 5]]:
/// If question 0 is solved, you will earn 3 points but you will be unable to solve questions 1 and 2.
/// If instead, question 0 is skipped and question 1 is solved, you will earn 4 points but you will be unable to solve questions 2 and 3.
/// Return the maximum points you can earn for the exam.
///
/// Example 1:
/// Input: questions = [[3,2],[4,3],[4,4],[2,5]]
/// Output: 5
/// Explanation: The maximum points can be earned by solving questions 0 and 3.
/// - Solve question 0: Earn 3 points, will be unable to solve the next 2 questions
/// - Unable to solve questions 1 and 2
/// - Solve question 3: Earn 2 points
/// Total points earned: 3 + 2 = 5. There is no other way to earn 5 or more points.
///
/// Example 2:
/// Input: questions = [[1,1],[2,2],[3,3],[4,4],[5,5]]
/// Output: 7
/// Explanation: The maximum points can be earned by solving questions 1 and 4.
/// - Skip question 0
/// - Solve question 1: Earn 2 points, will be unable to solve the next 2 questions
/// - Unable to solve questions 2 and 3
/// - Solve question 4: Earn 5 points
/// Total points earned: 2 + 5 = 7. There is no other way to earn 7 or more points.
///
/// Constraints:
/// 1 <= questions.length <= 105
/// questions[i].length == 2
/// 1 <= pointsi, brainpoweri <= 105
struct Solution {}

impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        // We can solve this problem with dynamic programming
        // Let dp[i] be the maximum points you can earn for the exam which consists of only questions from i onwards.
        let mut dp = vec![0i64; questions.len() + 1];
        for i in (0..questions.len()).rev() {
            let (point, power) = (questions[i][0], questions[i][1]);
            // If we skip this question, simple take as many points as possible from the previous attempt
            let skip = dp.get(i + 1).unwrap_or(&0) + 0;
            // If we take this question, we cannot take questions until i + power + 1. However, we get the point
            let take = dp.get(i + power as usize + 1).unwrap_or(&0) + point as i64;
            // Pick the best move for this particular question, the following questions already take care of themselves
            dp[i] = take.max(skip);
        }
        // Since we looped in reverse order, the very first element would be the result to the main problem
        dp[0]
    }
}

fn main() {
    assert_eq!(
        Solution::most_points(vec![vec![3, 2], vec![4, 3], vec![4, 4], vec![2, 5]]),
        5
    );
    assert_eq!(
        Solution::most_points(vec![
            vec![1, 1],
            vec![2, 2],
            vec![3, 3],
            vec![4, 4],
            vec![5, 5]
        ]),
        7
    );
}
