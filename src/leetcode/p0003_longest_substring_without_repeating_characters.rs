use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();

        let mut char_index_map: HashMap<&char, usize> = HashMap::new();
        let mut left_index = 0;

        let mut max_len = 0;
        for right_index in 0..s.len() {
            let ch = &s[right_index];

            if let Some(occur_index) = char_index_map.get(ch) {
                if *occur_index >= left_index {
                    left_index = *occur_index + 1;
                }
            }
            let len = right_index - left_index + 1;
            if len > max_len {
                max_len = len;
            }
            char_index_map.insert(ch, right_index);
        }
        max_len as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_basic() {
        let s = "abcabcbb".to_string();
        let len = Solution::length_of_longest_substring(s);
        assert_eq!(len, 3);

        let s = "pwwkew".to_string();
        let len = Solution::length_of_longest_substring(s);
        assert_eq!(len, 3);
    }

    #[test]
    fn test_empty_str() {
        let s = "".to_string();
        let len = Solution::length_of_longest_substring(s);
        assert_eq!(len, 0);
    }

    #[test]
    fn test_dup() {
        let s = "bbbbb".to_string();
        let len = Solution::length_of_longest_substring(s);
        assert_eq!(len, 1);
    }

    #[test]
    fn test_nodup() {
        let s = "abcdefg".to_string();
        let len = Solution::length_of_longest_substring(s);
        assert_eq!(len, 7);
    }

    #[test]
    fn test_postfix() {
        let s = "aab".to_string();
        let len = Solution::length_of_longest_substring(s);
        assert_eq!(len, 2);
    }

    #[test]
    fn test_() {
        let s = "abba".to_string();
        let len = Solution::length_of_longest_substring(s);
        assert_eq!(len, 2);
    }
}