// https://leetcode.com/problems/longest-substring-without-repeating-characters/

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut chars: [Option<usize>; 128] = [None; 128];

        let (mut left, mut right) = (0, 0);
        let mut res = 0;

        let s_bytes = s.as_bytes();

        while right < s_bytes.len() {
            let ch = s_bytes[right] as usize;

            if let Some(i) = chars[ch] {
                if left <= i && i < right {
                    left = i + 1;
                }
            }

            res = res.max(right - left + 1);
            chars[ch] = Some(right);

            right += 1;
        }

        res as i32
    }
}
