pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut index = 0;

        for i in 0..nums.len() {
            if nums[i] != val {
                nums[index] = nums[i];
                index += 1;
            }
        }
        index as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let mut nums = vec![3,2,2,3];
        let val = 3;
        let len = Solution::remove_element(&mut nums, val);

        assert_eq!(len, 2);
        assert_eq!(nums[0..len as usize], vec![2,2])
    }


    #[test]
    fn test_case2() {
        let mut nums = vec![0,1,2,2,3,0,4,2];
        let val = 2;
        let len = Solution::remove_element(&mut nums, val);

        assert_eq!(len, 5);
        assert_eq!(nums[0..len as usize], vec![0,1,3,0,4])
    }
}