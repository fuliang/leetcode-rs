use std::mem;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}


pub struct List {
    pub head: Option<Box<ListNode>>,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn init(list: Option<Box<ListNode>>) -> Self {
        List  { head: list }
    }

    pub fn add_all(&mut self, list: &Vec<i32>) {
        for i in (0..list.len()).rev() {
            self.push(list[i]);
        }
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
}