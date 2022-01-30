use crate::leetcode::link_node::*;

pub struct Solution;

impl Solution {
    pub fn get_len(head: &Option<Box<ListNode>>) -> i32 {
        let mut p = head;
        let mut i = 0;
        while p.is_some() {
            p = &p.as_ref().unwrap().next;
            i += 1;
        }
        return i;
    }

    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let len = Self::get_len(&head);
        if len < n {
            return head;
        }

        let mut dummy_head = Some(Box::new(ListNode {
            val: 0, next: head,
        }));

        let walk_steps = len - n;
        let mut p = dummy_head.as_mut();
        for _ in 0..walk_steps {
            p = p.unwrap().next.as_mut()
        }

        let r_node = p.as_mut().unwrap().next.as_mut();
        p.unwrap().next = r_node.unwrap().next.take();
        dummy_head.unwrap().next
    }
}

#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn test_get_len() {
        let mut list = List::new();
        let mut i = 5;
        while i >= 1 {
            list.push(i);
            i -= 1;
        }
        let len = Solution::get_len(&list.head);
        assert_eq!(len, 5);
    }

    #[test]
    fn test_case1() {
        let mut list = List::new();
        let mut i = 5;
        while i >= 1 {
            list.push(i);
            i -= 1;
        }

        let head_clone = list.head.clone();
        let head_ans = Solution::remove_nth_from_end(head_clone, 5);
        
        let list = List::init(head_ans);
        for i in list.iter() {
            println!("{}", i);
        }
    }

    #[test]
    fn test_take() {
        let mut p = Some(Box::new(
            ListNode {
                val: 0,
                next: Some (
                    Box::new(
                        ListNode {
                            val: 1,
                            next: None,
                        }
                    )
                )
            }
        ));
        let q = p.as_mut().unwrap().next.take();
        println!("{:?} {:?}", p, q);
    }
}