// https://leetcode.com/problems/multiply-strings/description/

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let result = if num1 == "0" || num2 == "0" {
            "0".to_string()
        } else {
            let mut res = vec![0; num1.len() + num2.len()];

            for (p2, d2) in num2.chars().rev().enumerate() {
                for (p1, d1) in num1.chars().rev().enumerate() {
                    let nz = p1 + p2;
                    let c = res[nz];
                    let mul = (d1.to_digit(10).unwrap() * d2.to_digit(10).unwrap()) + c;
                    res[nz] = mul % 10;
                    if nz + 1 < res.len() {
                        res[nz + 1] += mul / 10;
                    }
                }
            }
            while res.len() > 1 && res[res.len() - 1] == 0 {
                res.pop();
            }
            let result_str: String = res
                .iter()
                .rev()
                .map(|&d| char::from_digit(d as u32, 10).unwrap())
                .collect();

            result_str
        };
        result
    }
}
