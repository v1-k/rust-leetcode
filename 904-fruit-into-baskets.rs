// https://leetcode.com/problems/fruit-into-baskets/description/

use std::collections::HashMap;

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut max_fruits = 0;
        let mut basket: HashMap<i32, usize> = HashMap::new();
        for r in 0..fruits.len() {
            let fruit = fruits[r];
            *basket.entry(fruit).or_insert(0) += 1;

            while basket.len() > 2 {
                let left_fruit = fruits[l];
                let left_count = basket.get_mut(&left_fruit).unwrap();
                *left_count -= 1;
                if *left_count == 0 {
                    basket.remove(&left_fruit);
                }
                l += 1;
            }
            max_fruits = max_fruits.max((r - l + 1) as usize);
        }
        max_fruits as i32
    }
}
