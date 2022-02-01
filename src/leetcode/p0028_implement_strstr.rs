pub struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }

        if haystack.is_empty() || haystack.len() < needle.len() {
            return -1;
        }

        let mut next: Vec<i32> = Vec::new();  
        Solution::get_next(&mut next, &needle);

        let haystack: Vec<char> = haystack.chars().collect();
        let needle: Vec<char> = needle.chars().collect();
        
        let mut j = -1_i32;
        for i in 0..haystack.len() {
            while j >= 0 && haystack[i] != needle[(j+1) as usize] {
                j = next[j as usize];
            }

            if haystack[i] == needle[(j+1) as usize] {
                j += 1;
            }

            if j as usize == needle.len() - 1 {
                return (i - needle.len() + 1) as i32;
            }
        }
        -1
    }

    fn get_next(next: &mut Vec<i32>, s: &str) {
        let s: Vec<char> = s.chars().collect();
        let mut j = -1;

        next.push(j);
        for i in 1..s.len() {
            while j != -1 && s[i] != s[(j + 1) as usize] {
                j = next[j as usize];//backtrack
            }
            
            if s[i] == s[(j + 1) as usize] {
                j += 1;
            }
            next.push(j);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let (haystack, needle) = ("hello".to_string(), "ll".to_string());
        let index = Solution::str_str(haystack, needle);
        assert_eq!(index, 2);
    }

    #[test]
    fn test_case2() {
        let (haystack, needle) = ("aaaaa".to_string(), "bba".to_string());
        let index = Solution::str_str(haystack, needle);
        assert_eq!(index, -1);
    }
}