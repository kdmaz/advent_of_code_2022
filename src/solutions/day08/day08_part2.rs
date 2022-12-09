#![allow(clippy::needless_range_loop)]

fn main(input: &str) -> u32 {
    let nums: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    let row_len = nums[0].len() as u32;
    let col_len = nums.len() as u32;
    let mut max_scenic = 1;

    for row in 0..nums.len() {
        if row == 0 || row == row_len as usize - 1 {
            continue;
        }

        for col in 0..nums[row].len() {
            if col == 0 || col == col_len as usize - 1 {
                continue;
            }

            let num = nums[row][col];

            // top
            let mut top_val = 1;
            for top in (0..row).rev() {
                let val = nums[top][col];
                if num > val && top != 0 {
                    top_val += 1;
                } else {
                    break;
                }
            }

            // bottom
            let mut bottom_val = 1;
            for bottom in (row + 1)..row_len as usize {
                let val = nums[bottom][col];
                if num > val && bottom != (row_len - 1) as usize {
                    bottom_val += 1;
                } else {
                    break;
                }
            }

            // left
            let mut left_val = 1;
            for left in (0..col).rev() {
                let val = nums[row][left];
                if num > val && left != 0 {
                    left_val += 1;
                } else {
                    break;
                }
            }

            // right
            let mut right_val = 1;
            for right in (col + 1)..col_len as usize {
                let val = nums[row][right];
                if num > val && right != (col_len - 1) as usize {
                    right_val += 1;
                } else {
                    break;
                }
            }

            let scenic_score = top_val * bottom_val * left_val * right_val;
            max_scenic = std::cmp::max(scenic_score, max_scenic);
        }
    }

    max_scenic
}

#[cfg(test)]
mod tests {
    use super::main;
    use crate::read_file;

    #[test]
    fn example() {
        let input = read_file("examples", 8);
        let output = main(&input);
        let expected = 8;
        assert_eq!(output, expected);
    }

    #[test]
    fn input() {
        let input = read_file("input", 8);
        let output = main(&input);
        let expected = 444528;
        assert_eq!(output, expected);
    }
}
