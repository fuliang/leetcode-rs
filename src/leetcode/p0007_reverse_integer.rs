struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut result = 0_i32;
        let mut x = x;

        while x != 0 {
            let rem = x % 10;

            let r = result.checked_mul(10)
                    .map(|r| r.checked_add(rem))
                    .flatten();

            match r {
                None => {
                    result = 0;
                    break;
                },
                Some(r) => {
                    result = r;
                }
            }
            x /= 10;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case1() {
        let x = 123;
        let r_x = Solution::reverse(x);
        assert_eq!(r_x, 321);
    }

    #[test]
    fn test_case2() {
        let x = -123;
        let r_x = Solution::reverse(x);
        assert_eq!(r_x, -321);
    }

    #[test]
    fn test_case3() {
        let x = 1;
        let r_x = Solution::reverse(x);
        assert_eq!(r_x, x);
    }

    #[test]
    fn test_case4() {
        let x = 2147483647;
        let r_x = Solution::reverse(x);
        assert_eq!(r_x, 0);
    }
}