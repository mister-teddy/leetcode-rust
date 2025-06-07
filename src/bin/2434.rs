use std::collections::BTreeMap;

/// Category: algorithms
/// Level: Medium
/// Percent: 41.480984%

/// You are given a string s and a robot that currently holds an empty string t. Apply one of the following operations until s and t are both empty:
///
///
/// 	Remove the first character of a string s and give it to the robot. The robot will append this character to the string t.
/// 	Remove the last character of a string t and give it to the robot. The robot will write this character on paper.
///
///
/// Return the lexicographically smallest string that can be written on the paper.
///
///
/// Example 1:
///
/// Input: s = "zza"
/// Output: "azz"
/// Explanation: Let p denote the written string.
/// Initially p="", s="zza", t="".
/// Perform first operation three times p="", s="", t="zza".
/// Perform second operation three times p="azz", s="", t="".
///
///
/// Example 2:
///
/// Input: s = "bac"
/// Output: "abc"
/// Explanation: Let p denote the written string.
/// Perform first operation twice p="", s="c", t="ba".
/// Perform second operation twice p="ab", s="c", t="".
/// Perform first operation p="ab", s="", t="c".
/// Perform second operation p="abc", s="", t="".
///
///
/// Example 3:
///
/// Input: s = "bdda"
/// Output: "addb"
/// Explanation: Let p denote the written string.
/// Initially p="", s="bdda", t="".
/// Perform first operation four times p="", s="", t="bdda".
/// Perform second operation four times p="addb", s="", t="".
///
///
///
/// Constraints:
///
///
/// 	1 <= s.length <= 10⁵
/// 	s consists of only English lowercase letters.
///

impl Solution {
    pub fn robot_with_string(s: String) -> String {
        let mut res = vec![];
        let mut stack = vec![];
        let mut count = BTreeMap::new();
        for byte in s.bytes() {
            *count.entry(byte).or_insert(0) += 1;
        }
        for byte in s.bytes() {
            // Push each character to the stack
            stack.push(byte);
            // Update the counter
            count.entry(byte).and_modify(|v| *v -= 1);
            if *count.get(&byte).unwrap() == 0 {
                count.remove(&byte);
            }

            if let Some((smallest_to_the_right, _)) = count.first_key_value() {
                // Keep writing the characters down the paper as long as it is smaller than the element on the right
                while !stack.is_empty() && stack.last().unwrap() <= smallest_to_the_right {
                    res.push(stack.pop().unwrap());
                }
                // If the character on the right is smaller, we should continue waiting for it, which is appending to the stack, not writing to the paper
            };
        }
        stack.reverse();
        // Write all character left from the stack
        res.append(&mut stack);
        String::from_utf8(res).unwrap()
    }
}
