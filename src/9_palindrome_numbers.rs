// Given an integer x, return true if x is a , and false otherwise.

// Example 1:

// Input: x = 121
// Output: true
// Explanation: 121 reads as 121 from left to right and from right to left.

// Example 2:

// Input: x = -121
// Output: false
// Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.

// Example 3:

// Input: x = 10
// Output: false
// Explanation: Reads 01 from right to left. Therefore it is not a palindrome.

// Constraints:

//     -231 <= x <= 231 - 1

// Follow up: Could you solve it without converting the integer to a string?

struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        } else if x >= 0 && x < 10 {
            return true;
        } else {
            return Self::reverse_num(x);
        }
    }

    pub fn reverse_num(x: i32) -> bool {
        let mut copy_num = x;
        let mut reversed_num = 0;

        while copy_num != 0 {
            let digit = copy_num % 10;
            copy_num = copy_num / 10;
            reversed_num = reversed_num * 10 + digit;
        }

        if reversed_num == x {
            return true;
        } else {
            return false;
        }
    }
}

fn main() {
    let cases = vec![(121, true), (-121, false), (10, false)];

    for (num, expected) in cases {
        let result = Solution::is_palindrome(num);
        println!(
            "num: {:?}, result: {:?}, expected: {:?}",
            num, result, expected
        );
        assert_eq!(result, expected);
    }
}
