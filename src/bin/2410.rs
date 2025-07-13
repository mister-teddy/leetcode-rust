/// Category: algorithms
/// Level: Medium
/// Percent: 66.618576%

/// You are given a 0-indexed integer array players, where players[i] represents the ability of the ith player. You are also given a 0-indexed integer array trainers, where trainers[j] represents the training capacity of the jth trainer.
///
/// The ith player can match with the jth trainer if the player's ability is less than or equal to the trainer's training capacity. Additionally, the ith player can be matched with at most one trainer, and the jth trainer can be matched with at most one player.
///
/// Return the maximum number of matchings between players and trainers that satisfy these conditions.
///
///
/// Example 1:
///
/// Input: players = [4,7,9], trainers = [8,2,5,8]
/// Output: 2
/// Explanation:
/// One of the ways we can form two matchings is as follows:
/// - players[0] can be matched with trainers[0] since 4 <= 8.
/// - players[1] can be matched with trainers[3] since 7 <= 8.
/// It can be proven that 2 is the maximum number of matchings that can be formed.
///
///
/// Example 2:
///
/// Input: players = [1,1,1], trainers = [10]
/// Output: 1
/// Explanation:
/// The trainer can be matched with any of the 3 players.
/// Each player can only be matched with one trainer, so the maximum answer is 1.
///
///
///
/// Constraints:
///
///
/// 	1 <= players.length, trainers.length <= 10⁵
/// 	1 <= players[i], trainers[j] <= 10⁹
///
///
///
/// Note: This question is the same as  445: Assign Cookies.

impl Solution {
    pub fn match_players_and_trainers(players: Vec<i32>, trainers: Vec<i32>) -> i32 {
        let mut players = players;
        players.sort();
        let mut trainers = trainers;
        trainers.sort();
        let mut i = 0;
        let mut res = 0;
        for trainer in trainers {
            if players[i] <= trainer {
                res += 1;
                i += 1;
                if i == players.len() {
                    break;
                }
            }
        }
        res
    }
}
