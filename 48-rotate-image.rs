// https://leetcode.com/problems/rotate-image/description/

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let rows = matrix.len();
        let cols = matrix[0].len();

        for r in 0..rows {
            for c in r..cols {
                let temp = matrix[r][c];
                matrix[r][c] = matrix[c][r];
                matrix[c][r] = temp;
            }
        }

        let mut lc = 0;
        let mut rc = cols - 1;

        while lc < rc {
            for r in 0..rows {
                let temp = matrix[r][lc];
                matrix[r][lc] = matrix[r][rc];
                matrix[r][rc] = temp;
            }
            lc += 1;
            rc -= 1;
        }
    }
}
