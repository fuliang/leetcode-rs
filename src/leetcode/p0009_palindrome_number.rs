struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        match x {
            x if x < 0 => false,
            x if x < 10 => true,
            x if x % 10 == 0 => false,
            _ => {
                let mut x = x;
                let mut rev = 0;
                while x > rev {
                    rev = rev * 10 + x % 10;
                    x /= 10;
                }
                x == rev || x == rev / 10
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let x = 121;
        assert!(Solution::is_palindrome(x))
    }

    #[test]
    fn test_case2() {
        let x = -121;
        assert!(!Solution::is_palindrome(x))
    }

    #[test]
    fn test_case3() {
        let x = 10;
        assert!(!Solution::is_palindrome(x))
    }

    #[test]
    fn test_case5() {
        let x = 9;
        assert!(Solution::is_palindrome(x))
    }
}