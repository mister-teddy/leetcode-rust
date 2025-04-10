use std::cmp::Ordering;

/// Category: algorithms
/// Level: Hard
/// Percent: 25.55793%

/// You are given three integers start, finish, and limit. You are also given a 0-indexed string s representing a positive integer.
///
/// A positive integer x is called powerful if it ends with s (in other words, s is a suffix of x) and each digit in x is at most limit.
///
/// Return the total number of powerful integers in the range [start..finish].
///
/// A string x is a suffix of a string y if and only if x is a substring of y that starts from some index (including 0) in y and extends to the index y.length - 1. For example, 25 is a suffix of 5125 whereas 512 is not.
///
///
/// Example 1:
///
/// Input: start = 1, finish = 6000, limit = 4, s = "124"
/// Output: 5
/// Explanation: The powerful integers in the range [1..6000] are 124, 1124, 2124, 3124, and, 4124. All these integers have each digit <= 4, and "124" as a suffix. Note that 5124 is not a powerful integer because the first digit is 5 which is greater than 4.
/// It can be shown that there are only 5 powerful integers in this range.
///
///
/// Example 2:
///
/// Input: start = 15, finish = 215, limit = 6, s = "10"
/// Output: 2
/// Explanation: The powerful integers in the range [15..215] are 110 and 210. All these integers have each digit <= 6, and "10" as a suffix.
/// It can be shown that there are only 2 powerful integers in this range.
///
///
/// Example 3:
///
/// Input: start = 1000, finish = 2000, limit = 4, s = "3000"
/// Output: 0
/// Explanation: All integers in the range [1000..2000] are less than 3000, hence "3000" cannot be a suffix of any integer in this range.
///
///
///
/// Constraints:
///
///
/// 	1 <= start <= finish <= 10¹⁵
/// 	1 <= limit <= 9
/// 	1 <= s.length <= floor(log₁₀(finish)) + 1
/// 	s only consists of numeric digits which are at most limit.
/// 	s does not have leading zeros.

impl Solution {
    pub fn number_of_powerful_int(start: i64, finish: i64, limit: i32, s: String) -> i64 {
        // The result of this problem is A - L - G. Where:
        // - A: all the powerful numbers with at most the same length as finish
        // - L: the numbers in A that are strictly less than start
        // - G: the numbers in A that are strictly greater than finish
        let finish_length = finish.ilog10() + 1;
        let A = (limit as i64 + 1).pow(finish_length - s.len() as u32); // Discrete Mathematics
        let L = Self::count_recursively(Ordering::Less, &start.to_string(), limit, &s);
        let G = Self::count_recursively(Ordering::Greater, &finish.to_string(), limit, &s);

        println!("{A} - {L} - {G}");

        return A - L - G;
    }
    // This function count how many (powerful integers with at most the same number of digits as threshold) that are strictly less/greater than threshold.
    // For example, with threshold 5278, order = Greater, limit = 5, s = "14"
    // The powerful numbers with at most the same number of digits as threshold are: 0014, 0114, 0214, 0314, 0414, 0514, 1014, 1114, 1214, ..., 5214, 5314, 5414, 5514.
    // Of those numbers, only 5314, 5414, 5514 are strictly greater than threshold, so we return 3.
    // We can count this by recursing
    fn count_recursively(order: Ordering, threshold: &str, limit: i32, s: &str) -> i64 {
        // We cannot form any powerful numbers that have less digits than s, since all powerful numbers have s as a prefix.
        if threshold.len() < s.len() {
            return 0;
        }

        // This is the recursive base case
        // There is only 1 powerful number with s.length digit, that is s it self.
        // So if s is less/greater than threshold, we return 1. Otherwise, 0.
        if threshold.len() == s.len() {
            return if s.cmp(threshold) == order { 1 } else { 0 };
        };

        // Now this part is tricky. Let's focus only on the first digit of threshold.
        let d: i32 = threshold[..1].parse().unwrap(); // This is threshold's first digit

        // With threshold 5278, limit = 5, s = "14"
        // We will have these combinations of powerful numbers: **14
        // As the definition of powerful numbers, * can be one of [0..=5]
        // If the first * is 0, 1, 2, 3, 4, then **14 will ALWAYS be strictly less than threshold, regardless of whatever the second * is
        // So the total numbers in this case is: (The number of cases where the first * is strictly less/greater than threshold's first digit) * (The number of all powerful numbers that have 1 less digits than threshold)
        let valid_prefix_count = match order {
            Ordering::Less => d.min(limit + 1),
            Ordering::Greater => (limit - d).max(0),
            _ => 0,
        };
        let all_powerful_numbers_with_one_less_digit_count =
            (limit as i64 + 1).pow(threshold.len() as u32 - s.len() as u32 - 1);
        let first_case = valid_prefix_count as i64 * all_powerful_numbers_with_one_less_digit_count;

        // But if the first * is 5 (more generally: equal to the threshold's first digit), how many numbers do we have?
        // Answer: it depends on the other digits to the right. So we recursively call this function with a new threshold.
        let second_case = if d <= limit {
            let new_threshold = &threshold[1..]; // Left the first digit out.
            Self::count_recursively(order, new_threshold, limit, s)
        } else {
            0 // If the first digit of threshold falls out of limit, than no value of the first * can ever match it.
        };
        return first_case + second_case;
    }
}
