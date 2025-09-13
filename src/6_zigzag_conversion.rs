// The string "PAY P ALI SHIRING" is written in a zigzag pattern on a given number
// of rows like this: (you may want to display this pattern in a fixed font for better legibility)

// P   A   H   N
// A P L S I I G
// Y   I   R

// And then read line by line: "PAHNAPLSIIGYIR"

// Write the code that will take a string and make this conversion given a number of rows:
// string convert(string s, int numRows);

// Example 1:
// Input: s = "PAY P ALI S HIR I NG", numRows = 3
// Output: "PAHNAPLSIIGYIR"

// Example 2:
// Input: s = "PAYPALISHIRING", numRows = 4
// Output: "PINALSIGYAHRPI"

// Explaination:
// P     I     N
// A   L S   I G
// Y A   H R
// P     I

// Example 3:
// Input: s = "A", numRows = 1
// Output: "A"

// Program
struct Solution;

impl Solution {
    pub fn prepare_2d_chars(s: String, num_rows: i32) -> Vec<Vec<char>> {
        let mut str_2d: Vec<Vec<char>> = vec![];
        let num: usize = num_rows as usize;
        let mut is_odd: bool = true;
        let mut i: usize = 0;

        while i < s.len() {
             if is_odd {
                let (last_index, char_pads): (usize, usize) = if i + num < s.len() { (i + num, 0) } else {
                    let char_pads = i + num - s.len();
                    (s.len(), char_pads)
                };

                let sub_str = &s[i..last_index];
                let padded_str = if char_pads != 0 {
                        format!("{}{}", sub_str, "_".repeat(char_pads))
                    } else {
                        sub_str.to_string()
                    };

                str_2d.push(padded_str.chars().collect());

                is_odd = false;
                i = last_index;
             }
             else {
                let (last_index,char_pads): (usize, usize) = if i + (num - 2) < s.len() { (i + (num - 2), 0) } else {
                    let char_pads = i + (num - 2) - s.len();
                    (s.len(), char_pads)
                };
                
                let sub_str = &s[i..last_index];
                let padded_str = if char_pads != 0 {
                        format!("{}{}", sub_str, "_".repeat(char_pads))
                    } else {
                        sub_str.to_string()
                    };

                str_2d.push(padded_str.chars().collect());

                is_odd = true;
                i = last_index;
             }
        }

        // Add padding
        for (i, row) in str_2d.iter_mut().enumerate() {
             if i % 2 == 1 {
                row.insert(0, '_');
                row.push('_');
             }
        }

        str_2d
    }

    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let mut result: String = String::new();
        let str_2d: Vec<Vec<char>> = Solution::prepare_2d_chars(s, num_rows);
        println!("{:?}", str_2d);
        let mut left: usize = 0;
        let mut righti32: i32 = num_rows - 1;
        let mut is_odd: bool = true;

        let mut k: usize = 0;

        while left < num_rows as usize {
            let right: usize = righti32 as usize;
            let char_vec = &str_2d[k];
            println!("char vec: {:?}", char_vec);
            if is_odd {
                println!("char odd: {:?}", char_vec[left]);
                if char_vec[left] == '_'{
                result += &"".to_string();
                }
                else {
                    result += &char_vec[left].to_string();
                }
            }
            else {
                println!("char even: {:?}", char_vec[right]);
                if char_vec[right] == '_'{
                result += &"".to_string();
                }
                else {
                    result += &char_vec[right].to_string();
                }
            }

            println!("result: {:?}", result);
            k = (k + 1) % str_2d.len();
            if k == 0 {
                left += 1;
                righti32 -= 1;
                is_odd = true;
                println!("=====================");
            }
            else {
                is_odd = !is_odd;
            }
        }
        
        result
    }

    // copied from Microsoft Copilot
    // pub fn convert(s: String, num_rows: i32) -> String {
    //     if num_rows == 1 || s.len() <= num_rows as usize {
    //         return s;
    //     }

    //     let mut rows = vec![String::new(); num_rows as usize];
    //     let mut current_row = 0;
    //     let mut going_down = false;

    //     for c in s.chars() {
    //         rows[current_row].push(c);
    //         if current_row == 0 || current_row == (num_rows - 1) as usize {
    //             going_down = !going_down;
    //         }
    //         current_row = if going_down {
    //             current_row + 1
    //         } else {
    //             current_row - 1
    //         };
    //     }

    //     rows.concat()
    // }

}

fn main() {
    let sol = Solution::convert(String::from("AB"), 1);
    println!("Result: {}", sol);
}