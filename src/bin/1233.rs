use std::collections::HashMap;

/// Category: algorithms
/// Level: Medium
/// Percent: 75.70803%

/// Given a list of folders folder, return the folders after removing all sub-folders in those folders. You may return the answer in any order.
///
/// If a folder[i] is located within another folder[j], it is called a sub-folder of it. A sub-folder of folder[j] must start with folder[j], followed by a "/". For example, "/a/b" is a sub-folder of "/a", but "/b" is not a sub-folder of "/a/b/c".
///
/// The format of a path is one or more concatenated strings of the form: '/' followed by one or more lowercase English letters.
///
///
/// 	For example, "/leetcode" and "/leetcode/problems" are valid paths while an empty string and "/" are not.
///
///
///
/// Example 1:
///
/// Input: folder = ["/a","/a/b","/c/d","/c/d/e","/c/f"]
/// Output: ["/a","/c/d","/c/f"]
/// Explanation: Folders "/a/b" is a subfolder of "/a" and "/c/d/e" is inside of folder "/c/d" in our filesystem.
///
///
/// Example 2:
///
/// Input: folder = ["/a","/a/b/c","/a/b/d"]
/// Output: ["/a"]
/// Explanation: Folders "/a/b/c" and "/a/b/d" will be removed because they are subfolders of "/a".
///
///
/// Example 3:
///
/// Input: folder = ["/a/b/c","/a/b/ca","/a/b/d"]
/// Output: ["/a/b/c","/a/b/ca","/a/b/d"]
///
///
///
/// Constraints:
///
///
/// 	1 <= folder.length <= 4 * 10⁴
/// 	2 <= folder[i].length <= 100
/// 	folder[i] contains only lowercase letters and '/'.
/// 	folder[i] always starts with the character '/'.
/// 	Each folder name is unique.
///

impl Solution {
    pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
        // First we convert string path into string[] parts
        let folders = folder.iter().map(parts).collect();

        // Then we group related folders
        // The final result of this operation would be a map with non-sub-folders as key, and its sub-folders as values
        let mut map = HashMap::new();
        map.insert("".to_string(), folders);
        while organise(&mut map) {
            // repeat
        }

        // The answer to this problem is the keys of the map
        return map.into_keys().collect();
    }
}

fn parts(s: &String) -> Vec<String> {
    s.split("/").map(|s| s.to_string()).skip(1).collect()
}

/// This function goes one level deaper and groups folders together
fn organise(map: &mut HashMap<String, Vec<Vec<String>>>) -> bool {
    let mut to_be_continued = false;
    let mut new_map = HashMap::<String, Vec<Vec<String>>>::new();
    for (root, folders) in map.into_iter() {
        let should_skip_root = folders.is_empty() || folders.iter().any(|folder| folder.len() == 0);
        if should_skip_root {
            new_map.insert(root.clone(), vec![]);
            continue;
        }
        for folder in folders {
            let paths = folder;
            to_be_continued = true;
            let first_path = &paths[0]; // It is guaranteed that `paths` will never be empty
            let new_root = format!("{root}/{first_path}");
            new_map
                .entry(new_root)
                .or_insert(vec![])
                .push(paths[1..].to_vec());
        }
    }
    *map = new_map;
    to_be_continued
}
