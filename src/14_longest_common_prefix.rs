// Write a function to find the longest common prefix string amongst an array of strings.

// If there is no common prefix, return an empty string "".

 

// Example 1:

// Input: strs = ["flower","flow","flight"]
// Output: "fl"

// Example 2:

// Input: strs = ["dog","racecar","car"]
// Output: ""
// Explanation: There is no common prefix among the input strings.

 

// Constraints:

//     1 <= strs.length <= 200
//     0 <= strs[i].length <= 200
//     strs[i] consists of only lowercase English letters if it is non-empty.

struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_string();
        }

        let first = &strs[0];
        for (i, &byte) in first.as_bytes().iter().enumerate() {
            if strs.iter().skip(1).any(|s| i == s.len() || s.as_bytes()[i] != byte) {
                return first[..i].to_string();
            }
        }

        first.to_string()
    }
}

fn main() {
    assert_eq!(Solution::longest_common_prefix(vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]), "fl".to_string());
    assert_eq!(Solution::longest_common_prefix(vec!["dog".to_string(), "racecar".to_string(), "car".to_string()]), "".to_string());
    println!("All tests passed!");
}


// The choice to use .as_bytes() instead of .chars() is a critical performance optimization in Rust for this specific problem. Here is why:

//   1. Performance: $O(1)$ vs $O(n)$
//   In Rust, String is stored as UTF-8. Since UTF-8 is a variable-width encoding (characters can be 1 to 4 bytes long), you cannot index into a string like s[i]
//   to get a character.

//    * .chars().nth(i): To find the i-th character, Rust must start at the beginning of the string and count bytes until it reaches the i-th character. This makes
//      every lookup an $O(n)$ operation. In a nested loop, this turns an O(N × M) algorithm into an O(N × M²) algorithm.
//    * .as_bytes()[i]: Accessing a byte by index is a direct memory offset, which is an $O(1)$ operation.

//   2. Constraint Guarantees
//   Usually, working with bytes can be "dangerous" in Rust if the string contains multi-byte Unicode characters (like emojis or Kanji), because you might split a
//   character in half.

//   However, the problem constraints state:
//   > strs[i] consists of only lowercase English letters.

//   In UTF-8, all lowercase English letters (ASCII) are guaranteed to be exactly 1 byte. This means:
//    * The i-th character is exactly the i-th byte.
//    * The length in characters is exactly the length in bytes.

//   3. Summary of the Difference
//   By using as_bytes(), we treat the string as a simple array of numbers. This allows the CPU to perform the comparison extremely fast without the overhead of
//   Unicode decoding or linear scanning. 

//   ┌─────────────────┬─────────────┬──────────────────┐
//   │ Method          │ Access Time │ Total Complexity │
//   ├─────────────────┼─────────────┼──────────────────┤
//   │ .chars().nth(i) │ O(length)   │ O(N × M²)        │
//   │ .as_bytes()[i]  │ $O(1)$      │ O(N × M)         │
//   └─────────────────┴─────────────┴──────────────────┘

//   (Where N is the number of strings and M is the average string length.)
