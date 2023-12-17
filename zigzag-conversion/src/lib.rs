#[cfg(test)]
mod tests;

pub struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let num_rows = num_rows as usize;
        let mut matrix: Vec<Vec<char>> = vec![vec!['#'; s.len()]; num_rows];
        let [mut row, mut column] = [0, 0];
        let mut reverse = false;

        for c in s.chars() {
            matrix[row][column] = c;

            if reverse {
                row -= 1;
                column += 1;
                if row == 0 {
                    reverse = false;
                }
            } else {
                if row + 1 > num_rows - 1 && row > 0 {
                    row -= 1;
                    reverse = true;
                    column += 1;
                } else {
                    row += 1;
                }
            }
        }

        let mut result = "".to_owned();
        for word in &matrix {
            let linha: String = word.iter().collect();
            let linha = linha.replace("#", "");
            result = result + &linha;
        }

        result
    }
}
