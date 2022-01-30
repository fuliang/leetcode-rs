pub struct Solution;

fn char_to_int(c: char) -> i32 {
    match c {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
         _ => 0,
    }
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0;
        let ints: Vec<i32> = s.chars().map(|c| char_to_int(c)).collect();

        for i in 0 .. ints.len()-1 {
            if ints[i] >= ints[i+1] {
                result += ints[i]; 
            } else {
                result -= ints[i];
            }
        }
        result += ints[ints.len() - 1];
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_case1() {
        let s = "III";
        let i = Solution::roman_to_int(s.to_string());
        assert_eq!(i, 3);
    }

    #[test]
    fn test_case2() {
        let s = "IV";
        let i = Solution::roman_to_int(s.to_string());
        assert_eq!(i, 4);
    }

    #[test]
    fn test_case3() {
        let s = "IX";
        let i = Solution::roman_to_int(s.to_string());
        assert_eq!(i, 9);
    }

    #[test]
    fn test_case4() {
        let s = "LVIII";
        let i = Solution::roman_to_int(s.to_string());
        assert_eq!(i, 58);
    }

    #[test]
    fn test_case5() {
        let s = "MCMXCIV";
        let i = Solution::roman_to_int(s.to_string());
        assert_eq!(i, 1994);
    }
}