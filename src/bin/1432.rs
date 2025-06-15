use std::char;

/// Category: algorithms
/// Level: Medium
/// Percent: 40.257496%

/// You are given an integer num. You will apply the following steps to num two separate times:
///
///
/// 	Pick a digit x (0 <= x <= 9).
/// 	Pick another digit y (0 <= y <= 9). Note y can be equal to x.
/// 	Replace all the occurrences of x in the decimal representation of num by y.
///
///
/// Let a and b be the two results from applying the operation to num independently.
///
/// Return the max difference between a and b.
///
/// Note that neither a nor b may have any leading zeros, and must not be 0.
///
///
/// Example 1:
///
/// Input: num = 555
/// Output: 888
/// Explanation: The first time pick x = 5 and y = 9 and store the new integer in a.
/// The second time pick x = 5 and y = 1 and store the new integer in b.
/// We have now a = 999 and b = 111 and max difference = 888
///
///
/// Example 2:
///
/// Input: num = 9
/// Output: 8
/// Explanation: The first time pick x = 9 and y = 9 and store the new integer in a.
/// The second time pick x = 9 and y = 1 and store the new integer in b.
/// We have now a = 9 and b = 1 and max difference = 8
///
///
///
/// Constraints:
///
///
/// 	1 <= num <= 10⁸
///

impl Solution {
    pub fn max_diff(num: i32) -> i32 {
        fn max_of(num: i32) -> i32 {
            let s = num.to_string();
            if let Some(first_not_9) = s.find(|c| c != '9') {
                let char = s.chars().nth(first_not_9).unwrap();
                return s.replace(char, "9").parse().unwrap();
            }
            return num;
        }
        fn min_of(num: i32) -> i32 {
            let s = num.to_string();
            if let Some(first_char) = s.chars().next() {
                if first_char == '1' {
                    // Find the first >= 2 character and replace it with '0'
                    if let Some(first_gte_2) = s.find(|c| c != '0' && c != '1') {
                        let char = s.chars().nth(first_gte_2).unwrap();
                        return s.replace(char, "0").parse().unwrap();
                    }
                } else {
                    // Replace first char with '1'
                    return s.replace(first_char, "1").parse().unwrap();
                }
            }
            return num;
        }

        max_of(num) - min_of(num)
    }
}
