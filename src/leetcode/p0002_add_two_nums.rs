use std::mem;

use crate::leetcode::link_node::*;

struct List {
    head: Option<Box<ListNode>>,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn init(list: Option<Box<ListNode>>) -> Self {
        List  { head: list }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(ListNode{
            val: elem,
            next: mem::replace(&mut self.head, None)
        });
        self.head = Some(new_node);
    }


    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, None) {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.val)
            }
        }
    }
}

pub struct Iter<'a> {
    next: Option<&'a ListNode>
}

impl<'a> List {
    pub fn iter(&'a self) -> Iter<'a> {
        Iter { next: self.head.as_deref() }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.val
        })
    }
}

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
    fn basic() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        list.push(4);
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);

        for (i1, i2) in (0..10).zip(0..2) {
            println!("{}", i1 + i2);
        }
    }

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