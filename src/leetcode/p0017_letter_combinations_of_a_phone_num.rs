struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return Vec::new();
        }
        
        let nums: Vec<char> = digits.chars().collect();
        let chars_vec: Vec<Vec<char>> = nums.iter()
                                           .map(|c| Self::num_to_chars(*c))
                                           .collect();
        let mut ans = Vec::new();
        let mut combine = String::new();
        Self::backtrack(&chars_vec, &mut ans, 0, &mut combine);
        ans
    }

    pub fn backtrack(chars_vec: &Vec<Vec<char>>, ans: &mut Vec<String>, index: usize, combine: &mut String) {
        if index == chars_vec.len() {
            ans.push(combine.clone());
        } else {
            let chars = &chars_vec[index];
            for i in 0..chars.len() {
                combine.push(chars[i]);
                Self::backtrack(chars_vec, ans, index + 1, combine);
                combine.pop();
            }
        }
    }

    fn num_to_chars(num: char) -> Vec<char> {
        match num {
            '2' => vec!['a', 'b', 'c'],
            '3' => vec!['d', 'e', 'f'],
            '4' => vec!['g', 'h', 'i'],
            '5' => vec!['j', 'k', 'l'],
            '6' => vec!['m', 'n', 'o'],
            '7' => vec!['p', 'q', 'r', 's'],
            '8' => vec!['t', 'u', 'v'],
            '9' => vec!['w', 'x', 'y', 'z'],
             _ => vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_case1() {
        let digits = "23".into();
        let ans = Solution::letter_combinations(digits);
        assert_eq!(ans, vec!["ad","ae","af","bd","be","bf","cd","ce","cf"]);
    }

    #[test]
    fn test_case2() {
        let digits = "".into();
        let ans = Solution::letter_combinations(digits);
        let empty: Vec<String> = Vec::new();
        assert_eq!(ans, empty);
    }

    #[test]
    fn test_case3() {
        let digits = "2".into();
        let ans = Solution::letter_combinations(digits);
        assert_eq!(ans, ["a", "b", "c"]);
    }
}