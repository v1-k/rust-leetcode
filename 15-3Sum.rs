// https://leetcode.com/problems/3sum/description/

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut res: Vec<Vec<i32>> = Vec::new();
        for i in 0..nums.len() - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let mut l = i + 1;
            let mut r = nums.len() - 1;

            while l < r {
                let sum = nums[i] + nums[l] + nums[r];
                match sum {
                    n if n > 0 => r -= 1,
                    n if n < 0 => l += 1,
                    _ => {
                        res.push(vec![nums[i], nums[l], nums[r]]);
                        l += 1;
                        r -= 1;
                        while l < r && nums[l] == nums[l - 1] {
                            l += 1;
                        }
                    }
                }
            }
        }
        res
    }
}
