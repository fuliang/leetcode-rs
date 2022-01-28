struct Solution;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 4 {
            return Vec::new();
        }

        let mut nums = nums;
        nums.sort();
        let mut ans: Vec<Vec<i32>> = Vec::new();
        
        for i in 0..nums.len() - 3 {
            if i > 0 && nums[i] == nums[i-1] {
                continue;
            }

            for j in i + 1 .. nums.len() - 2 {
                if j > i + 1 && nums[j] == nums[j-1] {
                    continue;
                }

                let mut k = j + 1;
                let mut l = nums.len() - 1;

                while k < l {
                    let sum = nums[i] + nums[j] + nums[k] + nums[l];
                    if sum == target {
                        ans.push(vec![nums[i], nums[j], nums[k], nums[l]]);
                        k += 1;
                        while k + 1 < l && nums[k-1] == nums[k] {
                            k += 1;
                        }
                        l -= 1;
                        while l - 1 > k && nums[l] == nums[l+1] {
                            l = l - 1;
                        }
                    } else if sum < target {
                        k += 1;
                        while k + 1 < l && nums[k-1] == nums[k] {
                            k += 1;
                        }
                    } else if sum > target {
                        l = l - 1;
                        while l - 1 > k && nums[l] == nums[l+1] {
                            l -= 1;
                        }
                    }
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
        let nums = vec![1,0,-1,0,-2,2];
        let target = 0;
        let ans = Solution::four_sum(nums, target);
        let ans_expect = vec![
            vec![-2,-1,1,2],
            vec![-2,0,0,2],
            vec![-1,0,0,1]
        ];
        assert_eq!(ans, ans_expect);
    }

    #[test]
    fn test_case2() {
        let nums = vec![2,2,2,2,2,2];
        let target = 8;
        let ans = Solution::four_sum(nums, target);
        let ans_expect = vec![
            vec![2,2,2,2],
        ];
        assert_eq!(ans, ans_expect);
    }

    #[test]
    fn test_case3() {
        let nums = vec![-3,-2,-1,0,0,1,2,3];
        let target = 0;
        let ans = Solution::four_sum(nums, target);
     
        let ans_expect = vec![
            vec![-3,-2,2,3],
            vec![-3,-1,1,3],
            vec![-3,0,0,3],
            vec![-3,0,1,2],
            vec![-2,-1,0,3],
            vec![-2,-1,1,2],
            vec![-2,0,0,2],
            vec![-1,0,0,1]
        ];
        assert_eq!(ans, ans_expect);
    }
}