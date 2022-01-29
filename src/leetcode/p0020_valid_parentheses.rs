use std::{slice::SliceIndex, collections::HashMap};

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() % 2 == 1 {
            return false;
        }

        let lefts = vec!['(', '[', '{'];
        let rights = vec![')', ']', '}'];
        let pair: HashMap<char, char> = rights.into_iter().zip(lefts.into_iter()).collect();

        let mut stack: Vec<char> = Vec::new();
        for c in s.chars() {
            if let Some(left) = pair.get(&c) {
                let ch = stack.pop();

                if ch.is_none() {
                    return false;
                } else {
                    if ch.unwrap() != *left {
                        return false;
                    }
                }
            } else {
                stack.push(c);
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let str = "()".to_string();
        assert_eq!(Solution::is_valid(str), true);
    }

    #[test]
    fn test_case2() {
        let str = "()[]{}".to_string();
        assert_eq!(Solution::is_valid(str), true);
    }

    #[test]
    fn test_case3() {
        let str = "(]".to_string();
        assert_eq!(Solution::is_valid(str), false);
    }

    #[test]
    fn test_case4() {
        let str = "([)]".to_string();
        assert_eq!(Solution::is_valid(str), false);
    }

    #[test]
    fn test_case5() {
        let str = "{[]}".to_string();
        assert_eq!(Solution::is_valid(str), true);
    }

    #[test]
    fn test_case6() {
        let str = "((".to_string();
        assert_eq!(Solution::is_valid(str), false);
    }

    #[test]
    fn test_case7() {
        let str = "(){}}{".to_string();
        assert_eq!(Solution::is_valid(str), false);
    }
}