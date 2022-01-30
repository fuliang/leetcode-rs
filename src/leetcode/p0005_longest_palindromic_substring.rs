pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let len = s.len();
        if len < 2 {
            return s;
        }

        let s: Vec<char> = s.chars().collect();
        let mut dp = vec![vec![false; len]; len];
        
        let mut max_len = 1;
        let mut start = 0;

        for i in 0..len {
            dp[i][i] = true
        }

        for l in 2..=len {
            for i in 0..len {
                let j = l + i - 1;
                if j >= len {
                    break;
                }
                if s[i] == s[j] {
                    if j - i < 3 {
                        dp[i][j] = true;
                    } else {
                        dp[i][j] = dp[i + 1][j - 1];
                    }
                }
            
                if dp[i][j] && j - i + 1 > max_len {
                    max_len = j - i + 1;
                    start = i;
                }
            }
        }

       return s[start..start+max_len].iter().collect();
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases1() {
        let s = "babad".to_string();
        let result = Solution::longest_palindrome(s);
        assert_eq!(result, "bab".to_string());
    }

    #[test]
    fn test_cases2() {
        let s = "cbbd".to_string();
        let result = Solution::longest_palindrome(s);
        assert_eq!(result, "bb".to_string());
    }

    #[test]
    fn test_cases3() {
        let s = "a".to_string();
        let result = Solution::longest_palindrome(s);
        assert_eq!(result, "a".to_string());
    }

    #[test]
    fn test_cases4() {
        let s = "ac".to_string();
        let result = Solution::longest_palindrome(s);
        assert_eq!(result, "a".to_string());
    }
}