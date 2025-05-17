/// Category: algorithms
/// Level: Medium
/// Percent: 27.577642%

/// You are given a string array words, and an array groups, both arrays having length n.
///
/// The hamming distance between two strings of equal length is the number of positions at which the corresponding characters are different.
///
/// You need to select the longest subsequence from an array of indices [0, 1, ..., n - 1], such that for the subsequence denoted as [i₀, i₁, ..., ik-1] having length k, the following holds:
///
///
/// 	For adjacent indices in the subsequence, their corresponding groups are unequal, i.e., groups[ij] != groups[ij+1], for each j where 0 < j + 1 < k.
/// 	words[ij] and words[ij+1] are equal in length, and the hamming distance between them is 1, where 0 < j + 1 < k, for all indices in the subsequence.
///
///
/// Return a string array containing the words corresponding to the indices (in order) in the selected subsequence. If there are multiple answers, return any of them.
///
/// Note: strings in words may be unequal in length.
///
///
/// Example 1:
///
///
/// Input: words = ["bab","dab","cab"], groups = [1,2,2]
///
/// Output: ["bab","cab"]
///
/// Explanation: A subsequence that can be selected is [0,2].
///
///
/// 	groups[0] != groups[2]
/// 	words[0].length == words[2].length, and the hamming distance between them is 1.
///
///
/// So, a valid answer is [words[0],words[2]] = ["bab","cab"].
///
/// Another subsequence that can be selected is [0,1].
///
///
/// 	groups[0] != groups[1]
/// 	words[0].length == words[1].length, and the hamming distance between them is 1.
///
///
/// So, another valid answer is [words[0],words[1]] = ["bab","dab"].
///
/// It can be shown that the length of the longest subsequence of indices that satisfies the conditions is 2.
///
///
/// Example 2:
///
///
/// Input: words = ["a","b","c","d"], groups = [1,2,3,4]
///
/// Output: ["a","b","c","d"]
///
/// Explanation: We can select the subsequence [0,1,2,3].
///
/// It satisfies both conditions.
///
/// Hence, the answer is [words[0],words[1],words[2],words[3]] = ["a","b","c","d"].
///
/// It has the longest length among all subsequences of indices that satisfy the conditions.
///
/// Hence, it is the only answer.
///
///
///
/// Constraints:
///
///
/// 	1 <= n == words.length == groups.length <= 1000
/// 	1 <= words[i].length <= 10
/// 	1 <= groups[i] <= n
/// 	words consists of distinct strings.
/// 	words[i] consists of lowercase English letters.
///

impl Solution {
    pub fn get_words_in_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        fn valid_hamming_distance(w1: &String, w2: &String) -> bool {
            if w1.len() != w2.len() {
                return false;
            }
            let w2 = w2.as_bytes();
            w1.bytes().enumerate().filter(|&(i, b)| w2[i] != b).count() <= 1
        }
        // We can solve this problem with dynamic programming
        // let dp[i].0 be the longest subsequence that ends with words[i]
        // let dp[i].1 be the longest subsequence that does not have to end with words[i]
        let mut dp = vec![(vec![], vec![]); words.len()];
        for i in 0..words.len() {
            dp[i].0.push(words[i].clone());
            for j in 0..i {
                if groups[i] != groups[j] && valid_hamming_distance(&words[i], &words[j]) {
                    if dp[i].0.len() < dp[j].0.len() + 1 {
                        dp[i].0 = dp[j].0.clone();
                        dp[i].0.push(words[i].clone());
                    }
                }
            }

            dp[i].1 = if i == 0 || dp[i].0.len() > dp[i - 1].1.len() {
                &dp[i].0
            } else {
                &dp[i - 1].1
            }
            .clone();
        }
        dp[words.len() - 1].1.clone()
    }
}
