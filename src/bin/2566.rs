/// Category: algorithms
/// Level: Easy
/// Percent: 60.336304%

/// You are given an integer num. You know that Bob will sneakily remap one of the 10 possible digits (0 to 9) to another digit.
///
/// Return the difference between the maximum and minimum values Bob can make by remapping exactly one digit in num.
///
/// Notes:
///
///
/// 	When Bob remaps a digit d1 to another digit d2, Bob replaces all occurrences of d1 in num with d2.
/// 	Bob can remap a digit to itself, in which case num does not change.
/// 	Bob can remap different digits for obtaining minimum and maximum values respectively.
/// 	The resulting number after remapping can contain leading zeroes.
///
///
///
/// Example 1:
///
/// Input: num = 11891
/// Output: 99009
/// Explanation:
/// To achieve the maximum value, Bob can remap the digit 1 to the digit 9 to yield 99899.
/// To achieve the minimum value, Bob can remap the digit 1 to the digit 0, yielding 890.
/// The difference between these two numbers is 99009.
///
///
/// Example 2:
///
/// Input: num = 90
/// Output: 99
/// Explanation:
/// The maximum value that can be returned by the function is 99 (if 0 is replaced by 9) and the minimum value that can be returned by the function is 0 (if 9 is replaced by 0).
/// Thus, we return 99.
///
///
/// Constraints:
///
///
/// 	1 <= num <= 10⁸
///

impl Solution {
    pub fn min_max_difference(num: i32) -> i32 {
        fn digits_of(num: i32) -> Vec<i32> {
            // This function will return an array containing digits of num
            let mut digits = vec![];
            let mut num = num;
            while num > 0 {
                digits.insert(0, num % 10);
                num /= 10;
            }
            return digits;
        }

        fn to_num(digits: Vec<&i32>) -> i32 {
            // This function will convert the array of digits to the its number presentation
            let mut res = 0;
            for d in digits {
                res *= 10;
                res += d;
            }
            res
        }

        fn change_to(num: i32, digit: i32) -> i32 {
            // This function will find the first digit from the left that is differ from digit,
            // then change all occurences of that digit found to the digit passed as argument
            let digits = digits_of(num);
            if let Some(to_change) = digits.iter().find(|&&d| d != digit) {
                return to_num(
                    digits
                        .iter()
                        .map(|d| {
                            if d == to_change {
                                return &digit;
                            }
                            return d;
                        })
                        .collect(),
                );
            }
            num
        }

        change_to(num, 9) - change_to(num, 0)
    }
}
