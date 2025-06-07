/// Category: algorithms
/// Level: Medium
/// Percent: 76.58278%

/// You are given two strings of the same length s1 and s2 and a string baseStr.
///
/// We say s1[i] and s2[i] are equivalent characters.
///
///
/// 	For example, if s1 = "abc" and s2 = "cde", then we have 'a' == 'c', 'b' == 'd', and 'c' == 'e'.
///
///
/// Equivalent characters follow the usual rules of any equivalence relation:
///
///
/// 	Reflexivity: 'a' == 'a'.
/// 	Symmetry: 'a' == 'b' implies 'b' == 'a'.
/// 	Transitivity: 'a' == 'b' and 'b' == 'c' implies 'a' == 'c'.
///
///
/// For example, given the equivalency information from s1 = "abc" and s2 = "cde", "acd" and "aab" are equivalent strings of baseStr = "eed", and "aab" is the lexicographically smallest equivalent string of baseStr.
///
/// Return the lexicographically smallest equivalent string of baseStr by using the equivalency information from s1 and s2.
///
///
/// Example 1:
///
/// Input: s1 = "parker", s2 = "morris", baseStr = "parser"
/// Output: "makkek"
/// Explanation: Based on the equivalency information in s1 and s2, we can group their characters as [m,p], [a,o], [k,r,s], [e,i].
/// The characters in each group are equivalent and sorted in lexicographical order.
/// So the answer is "makkek".
///
///
/// Example 2:
///
/// Input: s1 = "hello", s2 = "world", baseStr = "hold"
/// Output: "hdld"
/// Explanation: Based on the equivalency information in s1 and s2, we can group their characters as [h,w], [d,e,o], [l,r].
/// So only the second letter 'o' in baseStr is changed to 'd', the answer is "hdld".
///
///
/// Example 3:
///
/// Input: s1 = "leetcode", s2 = "programs", baseStr = "sourcecode"
/// Output: "aauaaaaada"
/// Explanation: We group the equivalent characters in s1 and s2 as [a,o,e,r,s,c], [l,p], [g,t] and [d,m], thus all letters in baseStr except 'u' and 'd' are transformed to 'a', the answer is "aauaaaaada".
///
///
///
/// Constraints:
///
///
/// 	1 <= s1.length, s2.length, baseStr <= 1000
/// 	s1.length == s2.length
/// 	s1, s2, and baseStr consist of lowercase English letters.
///

impl Solution {
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        fn nth(byte: u8) -> usize {
            (byte - 'a' as u8) as usize
        }
        // We first need to build a array of each character's root
        let mut root: Vec<u8> = ('a' as u8..='z' as u8).collect();
        fn find_root(root: &mut Vec<u8>, c: u8) -> u8 {
            if c != root[nth(c)] {
                root[nth(c)] = find_root(root, root[nth(c)]);
            }
            root[nth(c)]
        }
        for (b1, b2) in s1.bytes().zip(s2.bytes()) {
            let r1 = find_root(&mut root, b1);
            let r2 = find_root(&mut root, b2);
            if r1 > r2 {
                // Update b1's root to r2
                // But before changing root, we must also change it's root's root
                root[nth(r1)] = r2;
            } else {
                root[nth(r2)] = r1;
            }
        }

        // Then we convert each character in base_str to its root
        String::from_utf8(base_str.bytes().map(|c| find_root(&mut root, c)).collect()).unwrap()
    }
}
