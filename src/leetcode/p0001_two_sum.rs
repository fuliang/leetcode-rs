use std::collections::HashMap;
pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let elem_to_index_map = nums.iter().zip(0..nums.len()).collect::<HashMap<&i32, usize>>();

        for (index, elem) in nums.iter().enumerate() {
            let diff = target - elem;

            if let Some(&result_index) = elem_to_index_map.get(&diff) {
                if result_index != index {
                    return vec![index as i32, result_index as i32];
                }
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_l1() {
        let nums = vec![2,7,11,15];
        let target = 9;
        let res = Solution::two_sum(nums, target);
        assert_eq!(res, vec![0, 1]);
    }

    #[test]
    fn test_empty_result() {
        let nums = vec![2, 7, 11, 15];
        let target = 10;
        let res = Solution::two_sum(nums, target);
        assert_eq!(res, vec![])
    }
}