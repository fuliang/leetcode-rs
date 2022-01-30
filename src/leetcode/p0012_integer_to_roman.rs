pub struct Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut num = num;
        let mut roman = String::new();

        let val_sym = vec![
            (1000, "M"),(900, "CM"),
            (500, "D"),(400, "CD"),
            (100, "C"),(90, "XC"),
            (50, "L"),(40, "XL"),
            (10, "X"),(9, "IX"),
            (5, "V"),(4, "IV"),
            (1, "I"),
        ];

        for (val, sym) in val_sym.iter() {
            while num >= *val {
                num -= *val;
                roman += sym;
            }

            if num == 0 {
                break;
            }
        }
        roman
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let num = 3;
        let roman = Solution::int_to_roman(num);
        assert_eq!(roman, "III");
    }

    #[test]
    fn test_case2() {
        let num = 4;
        let roman = Solution::int_to_roman(num);
        assert_eq!(roman, "IV");
    }

    #[test]
    fn test_case3() {
        let num = 9;
        let roman = Solution::int_to_roman(num);
        assert_eq!(roman, "IX");
    }

    #[test]
    fn test_case4() {
        let num = 58;
        let roman = Solution::int_to_roman(num);
        assert_eq!(roman, "LVIII");
    }

    #[test]
    fn test_case5() {
        let num = 1994;
        let roman = Solution::int_to_roman(num);
        assert_eq!(roman, "MCMXCIV");
    }
}