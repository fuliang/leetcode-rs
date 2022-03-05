pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut low = 0;
        let mut high = nums.len() - 1;
        let n = nums.len();

        let mut mid;
        while low <= high {
            mid = low + (high - low) / 2;
            if target == nums[mid] {
                return mid as i32;
            }

            if nums[0] <= nums[mid] {
                if nums[0] <= target && target < nums[mid] {
                    high = mid - 1;
                } else {
                    low = mid + 1;
                }
            } else {
                if nums[mid] < target && target <= nums[n-1] {
                    low = mid + 1;
                } else {
                    high = mid - 1;
                }
            }
        }
        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let nums = vec![4,5,6,7,0,1,2]; 
        let target = 0;
        let ans = Solution::search(nums, target);
        assert_eq!(ans, 4);
    }

    #[test]
    fn test_case2() {
        let nums = vec![4,5,6,7,0,1,2]; 
        let target = 3;
        let ans = Solution::search(nums, target);
        assert_eq!(ans, -1);
    }
}