use std::slice;

/// Category: algorithms
/// Level: Medium
/// Percent: 23.455872%

/// You are given a string word, and an integer numFriends.
///
/// Alice is organizing a game for her numFriends friends. There are multiple rounds in the game, where in each round:
///
///
/// 	word is split into numFriends non-empty strings, such that no previous round has had the exact same split.
/// 	All the split words are put into a box.
///
///
/// Find the lexicographically largest string from the box after all the rounds are finished.
///
///
/// Example 1:
///
///
/// Input: word = "dbca", numFriends = 2
///
/// Output: "dbc"
///
/// Explanation:
///
/// All possible splits are:
///
///
/// 	"d" and "bca".
/// 	"db" and "ca".
/// 	"dbc" and "a".
///
///
///
/// Example 2:
///
///
/// Input: word = "gggg", numFriends = 4
///
/// Output: "g"
///
/// Explanation:
///
/// The only possible split is: "g", "g", "g", and "g".
///
///
///
/// Constraints:
///
///
/// 	1 <= word.length <= 5 * 10³
/// 	word consists only of lowercase English letters.
/// 	1 <= numFriends <= word.length
///

impl Solution {
    pub fn answer_string(word: String, num_friends: i32) -> String {
        if num_friends == 1 {
            return word;
        }
        fn largest_prefix(slices: Vec<&str>) -> String {
            assert!(
                slices.len() > 0,
                "You must guarantee that `slices` has at least 1 element!"
            );
            let mut largests = vec![slices[0]];
            for slice in slices.iter().skip(1) {
                match slice.bytes().next().cmp(&largests[0].bytes().next()) {
                    std::cmp::Ordering::Less => {}
                    std::cmp::Ordering::Equal => {
                        largests.push(slice);
                    }
                    std::cmp::Ordering::Greater => largests = vec![slice],
                }
            }
            let suffixes: Vec<&str> = largests
                .iter()
                .map(|slice| &slice[1..])
                .filter(|slice| slice.len() > 0)
                .collect();
            if suffixes.len() == 0 {
                return largests[0].to_string();
            }
            let first_char = largests[0].chars().next().unwrap();
            return format!("{}{}", first_char, largest_prefix(suffixes));
        }

        let max_split_len = word.len() - num_friends as usize + 1;
        let slices = (0..word.len())
            .map(|start| &word[start..word.len().min(start + max_split_len)])
            .collect();
        largest_prefix(slices)
    }
}
