// You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.
// You may assume the two numbers do not contain any leading zero, except the number 0 itself.

struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy_head = ListNode::new(0);
        let mut current = &mut dummy_head;
        let mut p = l1;
        let mut q = l2;
        let mut carry = 0;

        while p.is_some() || q.is_some() || carry != 0 {
            let mut sum = carry;
            if let Some(node) = p {
                sum += node.val;
                p = node.next;
            }
            if let Some(node) = q {
                sum += node.val;
                q = node.next;
            }
            carry = sum / 10;
            current.next = Some(Box::new(ListNode::new(sum % 10)));
            current = current.next.as_mut().unwrap();
        }

        dummy_head.next
    }
}

fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for &val in vec.iter().rev() {
        let mut node = ListNode::new(val);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

fn to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut vec = Vec::new();
    while let Some(node) = head {
        vec.push(node.val);
        head = node.next;
    }
    vec
}

fn main() {
    let cases = vec![
        (vec![2, 4, 3], vec![5, 6, 4], vec![7, 0, 8]),
        (vec![0], vec![0], vec![0]),
        (
            vec![9, 9, 9, 9, 9, 9, 9],
            vec![9, 9, 9, 9],
            vec![8, 9, 9, 9, 0, 0, 0, 1],
        ),
    ];

    for (l1_vec, l2_vec, expected_vec) in cases {
        let l1 = to_list(l1_vec.clone());
        let l2 = to_list(l2_vec.clone());
        let result = Solution::add_two_numbers(l1, l2);
        let result_vec = to_vec(result);
        println!(
            "l1: {:?}, l2: {:?}, result: {:?}, expected: {:?}",
            l1_vec, l2_vec, result_vec, expected_vec
        );
        assert_eq!(result_vec, expected_vec);
    }
}
