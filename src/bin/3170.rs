use std::collections::{BTreeMap, BTreeSet};

/// Category: algorithms
/// Level: Medium
/// Percent: 35.221584%

/// You are given a string s. It may contain any number of '*' characters. Your task is to remove all '*' characters.
///
/// While there is a '*', do the following operation:
///
///
/// 	Delete the leftmost '*' and the smallest non-'*' character to its left. If there are several smallest characters, you can delete any of them.
///
///
/// Return the lexicographically smallest resulting string after removing all '*' characters.
///
///
/// Example 1:
///
///
/// Input: s = "aaba*"
///
/// Output: "aab"
///
/// Explanation:
///
/// We should delete one of the 'a' characters with '*'. If we choose s[3], s becomes the lexicographically smallest.
///
///
/// Example 2:
///
///
/// Input: s = "abc"
///
/// Output: "abc"
///
/// Explanation:
///
/// There is no '*' in the string.
///
///
///
/// Constraints:
///
///
/// 	1 <= s.length <= 10⁵
/// 	s consists only of lowercase English letters and '*'.
/// 	The input is generated such that it is possible to delete all '*' characters.
///

impl Solution {
    pub fn clear_stars(s: String) -> String {
        // This problem reduces to removing the last index of the smallest character visited so far
        // For efficient lookup, we use a BTreeMap of BTreeSet of indices of characters in s
        let mut count: BTreeMap<char, BTreeSet<usize>> = BTreeMap::new();
        let mut keep = vec![true; s.len()];
        for (i, char) in s.chars().enumerate() {
            if char == '*' {
                let mut it = count.first_entry().unwrap();
                let last_value = *it.get().last().unwrap();
                keep[last_value] = false;
                // Update the counter
                it.get_mut().pop_last();
                if it.get().is_empty() {
                    it.remove();
                }
            } else {
                let entry = count.entry(char).or_insert(BTreeSet::new());
                entry.insert(i);
            }
        }
        s.chars()
            .enumerate()
            .filter(|(i, c)| *c != '*' && keep[*i])
            .map(|e| e.1)
            .collect()
    }
}
