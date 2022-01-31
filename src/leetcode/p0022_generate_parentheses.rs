pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ans = Vec::new();
        Self::backtrack(&mut ans, &mut String::new(), 0, 0, n as usize);
        return ans;
    }

    pub fn backtrack(ans: &mut Vec<String>, cur: &mut String, open: usize, close: usize, n: usize) {
        if cur.len() == n * 2 {
            ans.push(cur.clone());
            return;
        }

        if open < n {
            cur.push('(');
            Self::backtrack(ans, cur, open + 1, close, n);
            cur.pop();
        }

        if close < open {
            cur.push(')');
            Self::backtrack(ans, cur, open, close + 1, n);
            cur.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
       let ans = Solution::generate_parenthesis(1);
       assert_eq!(ans, vec!["()"])
    }

    #[test]
    fn test_case2() {
        let ans = Solution::generate_parenthesis(3);
        let expected = vec![
            "((()))".to_string(),
            "(()())".to_string(),
            "(())()".to_string(),
            "()(())".to_string(),
            "()()()".to_string()];
            
        assert_eq!(ans, expected)
    }
}