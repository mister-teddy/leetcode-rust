/// Category: algorithms
/// Level: Medium
/// Percent: 44.235863%

/// You are given a string word and an integer k.
///
/// We consider word to be k-special if |freq(word[i]) - freq(word[j])| <= k for all indices i and j in the string.
///
/// Here, freq(x) denotes the frequency of the character x in word, and |y| denotes the absolute value of y.
///
/// Return the minimum number of characters you need to delete to make word k-special.
///
///
/// Example 1:
///
///
/// Input: word = "aabcaba", k = 0
///
/// Output: 3
///
/// Explanation: We can make word 0-special by deleting 2 occurrences of "a" and 1 occurrence of "c". Therefore, word becomes equal to "baba" where freq('a') == freq('b') == 2.
///
///
/// Example 2:
///
///
/// Input: word = "dabdcbdcdcd", k = 2
///
/// Output: 2
///
/// Explanation: We can make word 2-special by deleting 1 occurrence of "a" and 1 occurrence of "d". Therefore, word becomes equal to "bdcbdcdcd" where freq('b') == 2, freq('c') == 3, and freq('d') == 4.
///
///
/// Example 3:
///
///
/// Input: word = "aaabaaa", k = 2
///
/// Output: 1
///
/// Explanation: We can make word 2-special by deleting 1 occurrence of "b". Therefore, word becomes equal to "aaaaaa" where each letter's frequency is now uniformly 6.
///
///
///
/// Constraints:
///
///
/// 	1 <= word.length <= 10⁵
/// 	0 <= k <= 10⁵
/// 	word consists only of lowercase English letters.
///

impl Solution {
    pub fn minimum_deletions(word: String, k: i32) -> i32 {
        // First, we count the occurences of each character
        let mut occs = vec![0; 26];
        for char in word.chars() {
            occs[char as usize - b'a' as usize] += 1;
        }
        // Now, we consider each character's occurence as the lower bound
        // So the upper bound is occs[char] + k
        // So the result - minimum number of deletion - is the occurences that are below or above the 2 thresholds
        let mut res = i32::MAX;
        for anchor in 0..26 {
            let mut deletion = 0;
            for i in 0..26 {
                if occs[i] < occs[anchor] {
                    deletion += occs[i]
                }
                if occs[i] > occs[anchor] + k {
                    deletion += occs[i] - occs[anchor] - k;
                }
            }
            res = res.min(deletion);
        }
        res
    }
}
