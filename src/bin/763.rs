use std::{char, collections::HashMap};

/// 763. Partition Labels
///
/// You are given a string s. We want to partition the string into as many parts as possible so that each letter appears in at most one part. For example, the string "ababcc" can be partitioned into ["abab", "cc"], but partitions such as ["aba", "bcc"] or ["ab", "ab", "cc"] are invalid.
///
/// Note that the partition is done so that after concatenating all the parts in order, the resultant string should be s.
///
/// Return a list of integers representing the size of these parts.
///
/// # Example 1:
///
/// Input: s = "ababcbacadefegdehijhklij"
/// Output: [9,7,8]
/// Explanation:
/// The partition is "ababcbaca", "defegde", "hijhklij".
/// This is a partition so that each letter appears in at most one part.
/// A partition like "ababcbacadefegde", "hijhklij" is incorrect, because it splits s into less parts.
///
/// # Example 2:
///
/// Input: s = "eccbbbbdec"
/// Output: [10]
///
/// # Constraints:
///
/// - 1 <= s.length <= 500
/// - s consists of lowercase English letters.

struct Solution {}

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        // First, we calculate the last occurrence of each character
        let mut last_occurrence = HashMap::new();
        for (i, char) in s.chars().enumerate() {
            last_occurrence.insert(char, i);
        }

        // Then, we use a stack to push all characters between the current character
        // index and its last occurrence
        // Keep repeating the process until the stack is empty, then we found a slice point
        let chars: Vec<char> = s.chars().collect();
        let mut result = vec![];
        let mut ending = 0;
        while ending < s.len() {
            let mut new_ending = ending;
            let mut stack = vec![chars[new_ending]];
            while let Some(char) = stack.pop() {
                for i in new_ending..last_occurrence[&char] {
                    stack.push(chars[i]);
                }
                new_ending = new_ending.max(last_occurrence[&char] + 1);
            }
            result.push((new_ending - ending) as i32);
            ending = new_ending;
        }

        result
    }
}

fn main() {
    assert_eq!(Solution::partition_labels("ababcc".to_string()), vec![4, 2]);
    assert_eq!(
        Solution::partition_labels("ababcbacadefegdehijhklij".to_string()),
        vec![9, 7, 8]
    );
    assert_eq!(
        Solution::partition_labels("eccbbbbdec".to_string()),
        vec![10]
    );
    assert_eq!(Solution::partition_labels("a".to_string()), vec![1]);
    assert_eq!(Solution::partition_labels("a".repeat(500)), vec![500]);
}
