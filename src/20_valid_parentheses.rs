// Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
//
// An input string is valid if:
//
// Open brackets must be closed by the same type of brackets.
// Open brackets must be closed in the correct order.
// Every close bracket has a corresponding open bracket of the same type.
//
// Example 1:
//
// Input: s = "()"
//
// Output: true
//
// Example 2:
//
// Input: s = "()[]{}"
//
// Output: true
//
// Example 3:
//
// Input: s = "(]"
//
// Output: false
//
// Example 4:
//
// Input: s = "([])"
//
// Output: true
//
// Example 5:
//
// Input: s = "([)]"
//
// Output: false
//
//
//
// Constraints:
//
// 1 <= s.length <= 104
// s consists of parentheses only '()[]{}'.

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();

        for ch in s.chars() {
            match ch {
                '(' | '{' | '[' => {
                    stack.push(ch);
                }

                ')' => {
                    if stack.pop() != Some('(') {
                        return false;
                    }
                }

                '}' => {
                    if stack.pop() != Some('{') {
                        return false;
                    }
                }

                ']' => {
                    if stack.pop() != Some('[') {
                        return false;
                    }
                }

                _ => unreachable!(),
            }
        }

        stack.is_empty()
    }
}

fn main() {
    assert!(Solution::is_valid("()".to_string()));
    assert!(Solution::is_valid("()[]{}".to_string()));
    assert!(!Solution::is_valid("(]".to_string()));
    assert!(Solution::is_valid("([])".to_string()));
    assert!(!Solution::is_valid("([s]".to_string()));

    println!("All tests passed!");
}
