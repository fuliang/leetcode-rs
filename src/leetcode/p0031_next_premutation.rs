pub struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if nums.len() < 2 {
            return;
        }

        let mut i = nums.len() - 2;

        let mut is_max = false;
        while nums[i] >= nums[i + 1] {
            if i == 0 {
                is_max = true;
                break;
            } else {
                i -= 1;
            }
        }

        if !is_max {
            let mut j = nums.len() - 1;
            while nums[j] <= nums[i] {
                if j == 0 {
                    break;
                } else {
                    j -= 1;
                }
            }
            nums.swap(i, j);
        }

        let start = if is_max {i} else {i+1};
        Self::reverse_tail(nums, start);
    }

    fn reverse_tail(nums: &mut Vec<i32>, start: usize) {
        let mut i = start;
        let mut j = nums.len() - 1;
        while i < j {
            nums.swap(i, j);
            i += 1;
            j -= 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case1() {
        let mut nums = vec![1,2,3];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1,3,2]);
    }

    #[test]
    fn test_case2() {
        let mut nums = vec![3,2,1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1,2,3]);
    }

    #[test]
    fn test_case3() {
        let mut nums = vec![1,1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1,1]);
    }

    #[test]
    fn test_case4() {
        let mut nums = vec![4,5,2,6,3,1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![4,5,3,1,2,6]);
    }
}