pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        
        let mut start = 0;
        let mut end = 1_usize;
        
        while start < nums.len() {
            while end < nums.len() && nums[end] == nums[start] {
                end += 1;
            }

            if end >= nums.len() {
                break;
            }

            if end > start + 1 {
                nums[start + 1] = nums[end];
            }
            start += 1;
        }
        (start + 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_case1() {
        let mut nums = vec![0,0,1,1,1,2,2,3,3,4];
        let n = Solution::remove_duplicates(&mut nums);
        assert_eq!(n, 5);
        println!("{:?}", nums);
        assert_eq!(nums[0..5], vec![0,1,2,3,4])
    }
}