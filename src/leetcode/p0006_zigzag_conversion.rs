pub struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows < 2 {
            return s;
        }

        let rows = num_rows as usize;
        let s: Vec<char> = s.chars().collect();
        let mut str_vec = vec!["".to_string(); rows];

        let period = rows * 2 - 2;
        
        for i in 0..s.len() {
            let pmod = i % period;
            if pmod < rows {
                str_vec[pmod].push(s[i]);
            } else {
                str_vec[period - pmod].push(s[i]);
            }
        }
        str_vec.join("")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_case1() {
        let s = "PAYPALISHIRING".to_string();
        let num_rows = 3;
        let result = Solution::convert(s, num_rows);
        assert_eq!(result, "PAHNAPLSIIGYIR");
    }

    #[test]
    pub fn test_case2() {
        let s = "PAYPALISHIRING".to_string();
        let num_rows = 4;
        let result = Solution::convert(s, num_rows);
        assert_eq!(result, "PINALSIGYAHRPI");
    }

    #[test]
    pub fn test_case3() {
        let s = "A".to_string();
        let num_rows = 1;
        let result = Solution::convert(s, num_rows);
        assert_eq!(result, "A");
    }
}