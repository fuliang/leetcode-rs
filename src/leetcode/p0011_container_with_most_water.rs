struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = height.len() - 1;

        let mut max_area = 0_i32;
        while i < j {
            let area: i32 = i32::min(height[i], height[j]) * (j - i) as i32;
            if max_area < area {
                max_area = area;
            }
            if height[i] < height[j] {
                i += 1;
            } else {
                j -= 1;
            }
        }
        return max_area;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let height = vec![1,8,6,2,5,4,8,3,7];
        let area = Solution::max_area(height);
        assert_eq!(area, 49);
    }

    #[test]
    fn test_case2() {
        let height = vec![1,1];
        let area = Solution::max_area(height);
        assert_eq!(area, 1);
    }

    #[test]
    fn test_case3() {
        let height = vec![4,3,2,1,4];
        let area = Solution::max_area(height);
        assert_eq!(area, 16);
    }

    #[test]
    fn test_case4() {
        let height = vec![1,2,1];
        let area = Solution::max_area(height);
        assert_eq!(area, 2);
    }
}