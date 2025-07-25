/// Category: algorithms
/// Level: Medium
/// Percent: 62.83617%

/// You are given a string s and two integers x and y. You can perform two types of operations any number of times.
///
///
/// 	Remove substring "ab" and gain x points.
///
///
/// 		For example, when removing "ab" from "cabxbae" it becomes "cxbae".
///
///
/// 	Remove substring "ba" and gain y points.
///
/// 		For example, when removing "ba" from "cabxbae" it becomes "cabxe".
///
///
///
///
/// Return the maximum points you can gain after applying the above operations on s.
///
///
/// Example 1:
///
/// Input: s = "cdbcbbaaabab", x = 4, y = 5
/// Output: 19
/// Explanation:
/// - Remove the "ba" underlined in "cdbcbbaaabab". Now, s = "cdbcbbaaab" and 5 points are added to the score.
/// - Remove the "ab" underlined in "cdbcbbaaab". Now, s = "cdbcbbaa" and 4 points are added to the score.
/// - Remove the "ba" underlined in "cdbcbbaa". Now, s = "cdbcba" and 5 points are added to the score.
/// - Remove the "ba" underlined in "cdbcba". Now, s = "cdbc" and 5 points are added to the score.
/// Total score = 5 + 4 + 5 + 5 = 19.
///
/// Example 2:
///
/// Input: s = "aabbaaxybbaabb", x = 5, y = 4
/// Output: 20
///
///
///
/// Constraints:
///
///
/// 	1 <= s.length <= 10⁵
/// 	1 <= x, y <= 10⁴
/// 	s consists of lowercase English letters.
///

impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        // We solve this problem by greedily removing "ab" if x > y or "ba" otherwise
        let mut s = s;
        let mut res = 0;
        if x > y {
            // Remove "ab" first
            res += remove(&mut s, b"ab", x);
            res += remove(&mut s, b"ba", y);
        } else {
            // Remove "ba" first
            res += remove(&mut s, b"ba", y);
            res += remove(&mut s, b"ab", x);
        }
        res
    }
}

fn remove(s: &mut String, ab: &[u8; 2], x: i32) -> i32 {
    let mut bytes: Vec<u8> = s.bytes().collect();
    let mut start = -1i32;
    let mut end = 0i32;
    while (end as usize) < bytes.len() {
        if start >= 0 && bytes[start as usize] == ab[0] && bytes[end as usize] == ab[1] {
            start -= 1;
        } else {
            start += 1;
            bytes[start as usize] = bytes[end as usize];
        }
        end += 1;
    }
    bytes.truncate((start + 1) as usize);
    *s = String::from_utf8(bytes).unwrap();
    s.truncate((start + 1) as usize);
    return x * (end - start - 1) / 2;
}
