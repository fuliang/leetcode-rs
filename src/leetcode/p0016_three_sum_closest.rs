struct Solution;

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();

        let mut best = 100_000;

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i-1] {
                continue;
            }

            let mut j = i + 1;
            let mut k = nums.len() - 1;

            while k > j {
                let sum = nums[i] + nums[j] + nums[k];
                if sum == target {
                    return sum;
                }

                if i32::abs(sum - target) < i32::abs(best - target) {
                    best = sum;
                }

                if sum > target {
                    k = k - 1;
                    while k > j && nums[k+1] == nums[k] {
                        k = k - 1;
                    }
                } else {
                    j = j + 1;
                    while j < k && nums[j-1] == nums[j] {
                        j = j + 1;
                    }
                }
            }
        }
        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let nums = vec![-1, 2, 1, -4];
        let target = 1;
        let ans = Solution::three_sum_closest(nums, target);
        assert_eq!(ans, 2);
    }

    #[test]
    fn test_case2() {
        let nums = vec![-1, 2, 1, -4];
        let target = -1;
        let ans = Solution::three_sum_closest(nums, target);
        assert_eq!(ans, -1);
    }

    #[test]
    fn test_case3() {
        let nums = vec![0, 0, 0, 0];
        let target = 1;
        let ans = Solution::three_sum_closest(nums, target);
        assert_eq!(ans, 0);
    }
}