// You are given the heads of two sorted linked lists list1 and list2.
//
// Merge the two lists into one sorted list. The list should be made by splicing together the nodes of the first two lists.
//
// Return the head of the merged linked list.
//
//
//
// Example 1:
//
// Input: list1 = [1,2,4], list2 = [1,3,4]
// Output: [1,1,2,3,4,4]
//
// Example 2:
//
// Input: list1 = [], list2 = []
// Output: []
//
// Example 3:
//
// Input: list1 = [], list2 = [0]
// Output: [0]
//
//
//
// Constraints:
//
// The number of nodes in both lists is in the range [0, 50].
// -100 <= Node.val <= 100
// Both list1 and list2 are sorted in non-decreasing order.

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut p = list1;
        let mut q = list2;
        let mut dummy = None;
        let mut tail = &mut dummy;

        while p.is_some() && q.is_some() {
            let l1_val = p.as_ref().unwrap().val;
            let l2_val = q.as_ref().unwrap().val;

            if l1_val <= l2_val {
                *tail = p;
                p = tail.as_mut().unwrap().next.take();
            } else {
                *tail = q;
                q = tail.as_mut().unwrap().next.take();
            }
            tail = &mut tail.as_mut().unwrap().next;
        }

        *tail = if p.is_some() { p } else { q };

        dummy
    }
}

pub fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for &val in vec.iter().rev() {
        let mut node = ListNode::new(val);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

pub fn to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut vec = Vec::new();
    while let Some(node) = head {
        vec.push(node.val);
        head = node.next;
    }
    vec
}

fn main() {
    let cases = vec![
        (vec![1, 2, 4], vec![1, 3, 4], vec![1, 1, 2, 3, 4, 4]),
        (vec![], vec![], vec![]),
        (vec![], vec![0], vec![0]),
    ];

    for (l1_vec, l2_vec, expected_vec) in cases {
        let l1 = to_list(l1_vec.clone());
        let l2 = to_list(l2_vec.clone());
        let result = Solution::merge_two_lists(l1, l2);
        let result_vec = to_vec(result);
        println!(
            "l1: {:?}, l2: {:?}, result: {:?}, expected: {:?}",
            l1_vec, l2_vec, result_vec, expected_vec
        );
        assert_eq!(result_vec, expected_vec);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_merge_both_empty() {
        let l1 = to_list(vec![]);
        let l2 = to_list(vec![]);
        let result = Solution::merge_two_lists(l1, l2);
        assert_eq!(to_vec(result), vec![]);
    }

    #[test]
    fn test_merge_one_empty() {
        let l1 = to_list(vec![]);
        let l2 = to_list(vec![0]);
        let result = Solution::merge_two_lists(l1, l2);
        assert_eq!(to_vec(result), vec![0]);

        let l1 = to_list(vec![5]);
        let l2 = to_list(vec![]);
        let result = Solution::merge_two_lists(l1, l2);
        assert_eq!(to_vec(result), vec![5]);
    }

    #[test]
    fn test_merge_standard() {
        let l1 = to_list(vec![1, 2, 4]);
        let l2 = to_list(vec![1, 3, 4]);
        let result = Solution::merge_two_lists(l1, l2);
        assert_eq!(to_vec(result), vec![1, 1, 2, 3, 4, 4]);
    }

    #[test]
    fn test_merge_negative_numbers() {
        let l1 = to_list(vec![-10, -5, 0, 3]);
        let l2 = to_list(vec![-7, -2, 1, 4]);
        let result = Solution::merge_two_lists(l1, l2);
        assert_eq!(to_vec(result), vec![-10, -7, -5, -2, 0, 1, 3, 4]);
    }

    #[test]
    fn test_merge_different_lengths() {
        let l1 = to_list(vec![1]);
        let l2 = to_list(vec![2, 3, 4, 5]);
        let result = Solution::merge_two_lists(l1, l2);
        assert_eq!(to_vec(result), vec![1, 2, 3, 4, 5]);

        let l1 = to_list(vec![1, 3, 5, 7]);
        let l2 = to_list(vec![2]);
        let result = Solution::merge_two_lists(l1, l2);
        assert_eq!(to_vec(result), vec![1, 2, 3, 5, 7]);
    }
}
