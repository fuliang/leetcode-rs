use std::vec;

struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;

        nums.sort();
        if nums.len() < 3 {
            return vec![];
        }

        if nums[0] > 0 {
            return vec![]
        }

        let mut ans: Vec<Vec<i32>> = Vec::new();
        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i-1] {
                continue;
            } 

            let target = -nums[i];

            for j in i+1..nums.len() {
                if j > i+1 && nums[j] == nums[j-1] {
                    continue;
                }

                let mut k = nums.len() - 1;
                while k > j && nums[j] + nums[k] > target {
                    k -= 1;
                }

                if j == k {
                    break;
                }

                if nums[j] + nums[k] == target {
                    ans.push(vec![nums[i], nums[j], nums[k]])
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let nums = vec![-1,0,1,2,-1,-4];
        let ans = Solution::three_sum(nums);
        assert_eq!(ans, vec![vec![-1,-1,2],vec![-1,0,1]]);
    }

    #[test]
    fn test_case2() {
        let nums = vec![0];
        let ans = Solution::three_sum(nums);
        let empty: Vec<Vec<i32>> = Vec::new();
        assert_eq!(ans, empty);
    }

    #[test]
    fn test_case3() {
        let nums = vec![1,-1,-1,0];
        let ans = Solution::three_sum(nums);
        assert_eq!(ans, vec![vec![-1, 0, 1]]);
    }
}