// Given a string s, find the length of the longest without duplicate characters.

// Example 1:

// Input: s = "abcabcbb"
// Output: 3
// Explanation: The answer is "abc", with the length of 3. Note that "bca" and "cab" are also correct answers.

// Example 2:

// Input: s = "bbbbb"
// Output: 1
// Explanation: The answer is "b", with the length of 1.

// Example 3:

// Input: s = "pwwkew"
// Output: 3
// Explanation: The answer is "wke", with the length of 3.
// Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.

// Constraints:

//     0 <= s.length <= 5 * 104
//     s consists of English letters, digits, symbols and spaces.

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut char_indices = HashMap::new();
        let mut max_length = 0;
        let mut start = 0;

        for (end, c) in s.chars().enumerate() {
            if let Some(&prev_index) = char_indices.get(&c) {
                if prev_index >= start {
                    start = prev_index + 1;
                }
            }
            char_indices.insert(c, end);
            let current_length = end - start + 1;
            if current_length > max_length {
                max_length = current_length;
            }
        }

        max_length as i32
    }
}

fn main() {
    let cases = vec![
        (String::from("abcabcbb"), 3),
        (String::from("bbbbb"), 1),
        (String::from("pwwkew"), 3),
        (String::from("a"), 1),
        (String::from(""), 0),
        (String::from("dvdf"), 3),
        (String::from("abba"), 2),
        (String::from("tmmzuxt"), 5),
        (String::from(" "), 1),
    ];

    for (text, expected) in cases {
        let result = Solution::length_of_longest_substring(text.clone());
        println!(
            "text: {:?}, result: {:?}, expected: {:?}",
            text, result, expected
        );
        assert_eq!(result, expected);
    }
}
