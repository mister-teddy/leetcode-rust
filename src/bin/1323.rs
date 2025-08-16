use std::ops::IndexMut;

/// Category: algorithms
/// Level: Easy
/// Percent: 82.05356%

/// You are given a positive integer num consisting only of digits 6 and 9.
///
/// Return the maximum number you can get by changing at most one digit (6 becomes 9, and 9 becomes 6).
///
///
/// Example 1:
///
/// Input: num = 9669
/// Output: 9969
/// Explanation:
/// Changing the first digit results in 6669.
/// Changing the second digit results in 9969.
/// Changing the third digit results in 9699.
/// Changing the fourth digit results in 9666.
/// The maximum number is 9969.
///
///
/// Example 2:
///
/// Input: num = 9996
/// Output: 9999
/// Explanation: Changing the last digit 6 to 9 results in the maximum number.
///
///
/// Example 3:
///
/// Input: num = 9999
/// Output: 9999
/// Explanation: It is better not to apply any change.
///
///
///
/// Constraints:
///
///
/// 	1 <= num <= 10⁴
/// 	num consists of only 6 and 9 digits.
///

impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        // To get the maximum number, we swap the leftmost 6 with the rightmost 9
        let mut s: Vec<char> = num.to_string().chars().collect();
        let six = s.iter().position(|c| *c == '6');
        if let Some(six) = six {
            s[six] = '9';
        }
        return s.iter().collect::<String>().parse().unwrap();
    }
}
