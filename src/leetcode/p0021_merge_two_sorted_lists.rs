use crate::leetcode::link_node::*;

pub struct Solution;

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if list1.is_none() {
            return list2;
        }
        if list2.is_none() {
            return list1;
        }

        let mut p = list1;
        let mut q = list2;
        
        let mut dummy_head: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
        let mut prev = dummy_head.as_mut().unwrap();

        while p.is_some() && q.is_some(){
            if p.as_ref().unwrap().val < q.as_ref().unwrap().val {
                prev.next = p.clone();
                p = p.unwrap().next;
            } else {
                prev.next = q.clone();
                q = q.unwrap().next;
            }
            prev = prev.next.as_mut().unwrap();
        }

        while p.is_some() {
            prev.next = p.clone();
            p = p.unwrap().next;
            prev = prev.next.as_mut().unwrap();
        }
        
        while q.is_some() {
            prev.next = q.clone();
            q = q.unwrap().next;
            prev = prev.next.as_mut().unwrap();
        }
        dummy_head.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let v1 = vec![1,2,4];
        let v2 = vec![1,3,4];
        let mut l1 = List::new();
        l1.add_all(&v1);
        let mut l2 = List::new();
        l2.add_all(&v2);
        
        let ans = Solution::merge_two_lists(l1.head, l2.head);
        let l = List::init(ans);
        let ans_vec: Vec<&i32> = l.iter().collect();
        assert_eq!(ans_vec, vec![&1,&1,&2,&3,&4,&4]);
    }
}