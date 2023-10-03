use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::<i32, usize>::new();
        for i in 0..nums.len() {
            let num = nums[i];
            match map.get(&(target - num)) {
                Some(num) => return vec![i as i32, *num as i32],
                None => map.insert(num, i),
            };
        }
        return Vec::new();
    }
}
