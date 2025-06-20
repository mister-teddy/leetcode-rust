/// Category: algorithms
/// Level: Medium
/// Percent: 30.636131%

/// You are given a string s consisting of the characters 'N', 'S', 'E', and 'W', where s[i] indicates movements in an infinite grid:
///
///
/// 	'N' : Move north by 1 unit.
/// 	'S' : Move south by 1 unit.
/// 	'E' : Move east by 1 unit.
/// 	'W' : Move west by 1 unit.
///
///
/// Initially, you are at the origin (0, 0). You can change at most k characters to any of the four directions.
///
/// Find the maximum Manhattan distance from the origin that can be achieved at any time while performing the movements in order.
/// The Manhattan Distance between two cells (xi, yi) and (xj, yj) is |xi - xj| + |yi - yj|.
///
/// Example 1:
///
///
/// Input: s = "NWSE", k = 1
///
/// Output: 3
///
/// Explanation:
///
/// Change s[2] from 'S' to 'N'. The string s becomes "NWNE".
///
///
///
///
/// 			Movement
/// 			Position (x, y)
/// 			Manhattan Distance
/// 			Maximum
///
///
///
///
/// 			s[0] == 'N'
/// 			(0, 1)
/// 			0 + 1 = 1
/// 			1
///
///
/// 			s[1] == 'W'
/// 			(-1, 1)
/// 			1 + 1 = 2
/// 			2
///
///
/// 			s[2] == 'N'
/// 			(-1, 2)
/// 			1 + 2 = 3
/// 			3
///
///
/// 			s[3] == 'E'
/// 			(0, 2)
/// 			0 + 2 = 2
/// 			3
///
///
///
///
/// The maximum Manhattan distance from the origin that can be achieved is 3. Hence, 3 is the output.
///
///
/// Example 2:
///
///
/// Input: s = "NSWWEW", k = 3
///
/// Output: 6
///
/// Explanation:
///
/// Change s[1] from 'S' to 'N', and s[4] from 'E' to 'W'. The string s becomes "NNWWWW".
///
/// The maximum Manhattan distance from the origin that can be achieved is 6. Hence, 6 is the output.
///
///
///
/// Constraints:
///
///
/// 	1 <= s.length <= 10⁵
/// 	0 <= k <= s.length
/// 	s consists of only 'N', 'S', 'E', and 'W'.
///

impl Solution {
    pub fn max_distance(s: String, k: i32) -> i32 {
        // We can solve this problem with greedy
        // The maximum Manhattan distance from the origin can be found by:
        // - Keep a "dominant" direction
        // - And flip the opposite one for as much as possible
        let calc = |(_0, _1, _2, _3)| {
            let mut res = 0;
            let mut k = k;
            let flip = &mut |a: i32, b: i32| {
                let (max, min) = (a.max(b), a.min(b));
                // Take the distance of the "dominant" direction
                res += max;
                // Flip as many as k occurences of the opposite direction
                let flipable = k.min(min);
                res += flipable;
                res -= min - flipable;
                k -= flipable;
            };
            flip(_0, _1);
            flip(_2, _3);
            res
        };
        let mut occurences = (0, 0, 0, 0);
        let mut res = 0;
        for direction in s.chars() {
            let o = match direction {
                'N' => &mut occurences.0,
                'S' => &mut occurences.1,
                'E' => &mut occurences.2,
                'W' => &mut occurences.3,
                _ => panic!("s consists of only 'N', 'S', 'E', and 'W'."),
            };
            *o += 1;
            // We check the result for each occurence counted
            res = res.max(calc(occurences));
        }
        res
    }
}
