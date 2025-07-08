use std::{cmp::Reverse, collections::BinaryHeap};

/// Category: algorithms
/// Level: Medium
/// Percent: 32.84052%

/// You are given an array of events where events[i] = [startDayi, endDayi]. Every event i starts at startDayi and ends at endDayi.
///
/// You can attend an event i at any day d where startTimei <= d <= endTimei. You can only attend one event at any time d.
///
/// Return the maximum number of events you can attend.
///
///
/// Example 1:
///
/// Input: events = [[1,2],[2,3],[3,4]]
/// Output: 3
/// Explanation: You can attend all the three events.
/// One way to attend them all is as shown.
/// Attend the first event on day 1.
/// Attend the second event on day 2.
/// Attend the third event on day 3.
///
///
/// Example 2:
///
/// Input: events= [[1,2],[2,3],[3,4],[1,2]]
/// Output: 4
///
///
///
/// Constraints:
///
///
/// 	1 <= events.length <= 10⁵
/// 	events[i].length == 2
/// 	1 <= startDayi <= endDayi <= 10⁵
///

impl Solution {
    pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
        let max_day = events.iter().max_by_key(|e| e[1]).unwrap()[1];
        let mut sorted = events;
        sorted.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut attendable_events = BinaryHeap::new();
        let mut res = 0;
        let mut i = 0;
        for today in 0..=max_day {
            while i < sorted.len() && sorted[i][0] == today {
                attendable_events.push(Reverse(sorted[i][1]));
                i += 1;
            }
            loop {
                if let Some(Reverse(soonest_end)) = attendable_events.pop() {
                    if soonest_end < today {
                        continue;
                    } else {
                        res += 1;
                        break;
                    }
                } else {
                    break;
                }
            }
        }
        res
    }
}
