use std::collections::HashMap;

/// Category: algorithms
/// Level: Hard
/// Percent: 42.11067%

/// You are given two 0-indexed arrays nums1 and nums2 of length n, both of which are permutations of [0, 1, ..., n - 1].
///
/// A good triplet is a set of 3 distinct values which are present in increasing order by position both in nums1 and nums2. In other words, if we consider pos1v as the index of the value v in nums1 and pos2v as the index of the value v in nums2, then a good triplet will be a set (x, y, z) where 0 <= x, y, z <= n - 1, such that pos1x < pos1y < pos1z and pos2x < pos2y < pos2z.
///
/// Return the total number of good triplets.
///
///
/// Example 1:
///
/// Input: nums1 = [2,0,1,3], nums2 = [0,1,2,3]
/// Output: 1
/// Explanation:
/// There are 4 triplets (x,y,z) such that pos1x < pos1y < pos1z. They are (2,0,1), (2,0,3), (2,1,3), and (0,1,3).
/// Out of those triplets, only the triplet (0,1,3) satisfies pos2x < pos2y < pos2z. Hence, there is only 1 good triplet.
///
///
/// Example 2:
///
/// Input: nums1 = [4,0,1,3,2], nums2 = [4,1,0,2,3]
/// Output: 4
/// Explanation: The 4 good triplets are (4,0,3), (4,0,2), (4,1,3), and (4,1,2).
///
///
///
/// Constraints:
///
///
/// 	n == nums1.length == nums2.length
/// 	3 <= n <= 10⁵
/// 	0 <= nums1[i], nums2[i] <= n - 1
/// 	nums1 and nums2 are permutations of [0, 1, ..., n - 1].
///

impl Solution {
    pub fn good_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let pos1 = to_pos(&nums1);
        let pos2 = to_pos(&nums2);

        (0..nums1.len() - 2)
            .map(|start| count(start, 3, &nums1, &nums2, &pos1, &pos2))
            .sum()
    }
}

fn to_pos(nums: &Vec<i32>) -> HashMap<&i32, usize> {
    nums.iter()
        .enumerate()
        .fold(HashMap::new(), |mut set, (i, num)| {
            set.insert(num, i);
            set
        })
}

// This function return how many possible "sub-triplets" starting from index `start` are there
// A "sub-triplet" of length `count` is a collection of `count` numbers that is a suffix to a valid triplet
fn count(
    start: usize,
    sub_triplet: usize,
    nums: &Vec<i32>,
    other_nums: &Vec<i32>,
    pos: &HashMap<&i32, usize>,
    other_pos: &HashMap<&i32, usize>,
) -> i64 {
    if sub_triplet == 1 {
        return 1;
    }

    // Find nums[start] equivalent position in other_nums
    let start_pos_in_other_nums = other_pos[&nums[start]];

    let mut res = 0;
    // Each next number we find, must take place after the previous finding
    for i in start + 1..nums.len() {
        let i_pos_in_other_nums = other_pos[&nums[i]];
        if i_pos_in_other_nums > start_pos_in_other_nums {
            res += count(i, sub_triplet - 1, nums, other_nums, pos, other_pos);
        }
    }

    res
}
