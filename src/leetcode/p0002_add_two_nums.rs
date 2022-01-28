
use crate::leetcode::link_node::{ListNode, List};
use std::ops::Add;

impl Add for List {
    type Output = List;
    fn add(self, rhs: Self) -> Self {
        let mut head = Box::new(ListNode::new(0));
        let mut current = &mut head;
        let mut iter1 = self.iter();
        let mut iter2 = rhs.iter();

        let mut carry_pre = 0;

        loop{
            let (sum, carry) = match (iter1.next(), iter2.next(), carry_pre) {
                (Some(num1), Some(num2), carry) => {
                    ((num1 + num2 + carry) % 10, (num1 + num2 + carry) / 10)
                },
                (Some(num1), None, carry) => {
                    ((num1 + carry) % 10, (num1 + carry) / 10)
                },
                (None, Some(num2), carry) => {
                    ((num2 + carry) % 10, (num2 + carry) / 10)
                },
                (None, None, carry) => {
                    (carry, -1_i32)
                }
            };
            carry_pre = carry;
            if carry_pre == -1 {
                if sum != 0 {
                    current.next = Some(Box::new(ListNode::new(sum)));
                    current = current.next.as_mut().unwrap();
                }
                break;
            } else {
                current.next = Some(Box::new(ListNode::new(sum)));
                current = current.next.as_mut().unwrap();
            }
        }
        List::init(head.next)
    }
}
struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let l1 = List::init(l1);
        let l2 = List::init(l2);
        let result = l1 + l2;

        result.head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        // let num1 = [2,4,3];
        // let num2 = [5,6,4];
        let num1 = [9,9,9,9,9,9,9];
        let num2 = [9,9,9,9];

        let mut l1 = List::new();
        for i in num1.iter() {
            l1.push(*i);
        }
        
        let mut l2 = List::new();
        for i in num2.iter() {
            l2.push(*i);
        }

        let sum = Solution::add_two_numbers(l1.head, l2.head);
        let result = List::init(sum);
        for i in result.iter() {
            println!("{}", i)
        }
    }
}