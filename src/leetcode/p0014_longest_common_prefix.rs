struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() < 2 {
            return strs[0].clone();
        }

        let strs: Vec<Vec<char>> = strs.iter()
                                    .map(|f| f.chars().collect::<Vec<char>>())
                                    .collect();
        let first = &strs[0];
        
        let mut i = 0;
        let mut j = 0;
        loop {
                i = 1;
                while i < strs.len() {
                    if j < strs[i].len() && j < first.len() && strs[i][j] == first[j] {
                        i += 1;
                    } else {
                        break;
                    }
                }

                if i != strs.len() {
                    break;
                }
                j += 1;
        }
        return first[0..j].into_iter().collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_case1() {
        let strs = vec!["flower".to_string(),"flow".to_string(),"flight".to_string()];
        let common_prefix = Solution::longest_common_prefix(strs);
        assert_eq!(common_prefix, "fl");
    }

    #[test]
    pub fn test_case2() {
        let strs = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
        let common_prefix = Solution::longest_common_prefix(strs);
        assert_eq!(common_prefix, "");
    }
}